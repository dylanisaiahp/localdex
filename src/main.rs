use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::{Result, bail};
use colored::Colorize;
use dirs::home_dir;
use ignore::{DirEntry, WalkBuilder, WalkState};
use pico_args::Arguments;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// ---------------------------------------------------------------------------
// Config file structures
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FlagDef {
    short: String,
    long: String,
    description: String,
    os: String,
}

#[derive(Debug, Deserialize)]
struct LdxConfig {
    #[serde(default)]
    flags: HashMap<String, FlagDef>,
}

const DEFAULT_CONFIG: &str = r#"# ldx configuration file
# Edit this file to customise flags and behaviour
#
# os values: "all", "windows", "linux", "macos"

[flags.all-files]
short = "a"
long = "all-files"
description = "Count all files, no filter needed"
os = "all"

[flags.all-drives]
short = "A"
long = "all-drives"
description = "Scan all drives with a per-drive breakdown and total"
os = "windows"

[flags.dir]
short = "d"
long = "dir"
description = "Directory to search (default: current)"
os = "all"

[flags.extension]
short = "e"
long = "extension"
description = "Search by file extension (e.g. pdf, rs)"
os = "all"

[flags.first]
short = "1"
long = "first"
description = "Stop after the first match"
os = "all"

[flags.goto]
short = "g"
long = "goto"
description = "Print the cd command to navigate to the matched file"
os = "all"

[flags.help]
short = "h"
long = "help"
description = "Show this help message"
os = "all"

[flags.limit]
short = "L"
long = "limit"
description = "Stop after N matches (e.g. -L 5)"
os = "all"

[flags.open]
short = "o"
long = "open"
description = "Open or launch the matched file"
os = "all"

[flags.quiet]
short = "q"
long = "quiet"
description = "Suppress per-file output; still prints summary count"
os = "all"

[flags.case-sensitive]
short = "s"
long = "case-sensitive"
description = "Case-sensitive search"
os = "all"

[flags.stats]
short = "S"
long = "stats"
description = "Show scan statistics"
os = "all"

[flags.threads]
short = "t"
long = "threads"
description = "Number of threads to use (default: all available)"
os = "all"

[flags.verbose]
short = "v"
long = "verbose"
description = "Show detailed scan breakdown (files + dirs separately)"
os = "all"
"#;

fn load_config() -> Result<LdxConfig> {
    let config_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("config.toml");

    if !config_path.exists() {
        std::fs::write(&config_path, DEFAULT_CONFIG)?;
    }

    let contents = std::fs::read_to_string(&config_path)?;
    let config: LdxConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn is_flag_available(flag: &FlagDef) -> bool {
    match flag.os.as_str() {
        "all" => true,
        #[cfg(windows)]
        "windows" => true,
        #[cfg(target_os = "linux")]
        "linux" => true,
        #[cfg(target_os = "macos")]
        "macos" => true,
        _ => false,
    }
}

fn print_help(config: &LdxConfig) {
    println!(
        "Usage: ldx [pattern] [options]\n\
        \n\
        Examples:\n\
          ldx invoice.pdf                  # basename substring search\n\
          ldx -e pdf -q                    # count all .pdf files quietly\n\
          ldx rs -d D:\\Development        # find files with 'rs' in name\n\
          ldx -a -S -d C:\\               # count every file on C:\\\n\
          ldx -a -A -S                     # count every file on all drives\n\
        \n\
        Options:"
    );

    let mut flags: Vec<&FlagDef> = config
        .flags
        .values()
        .filter(|f| is_flag_available(f))
        .collect();

    flags.sort_by(|a, b| a.long.cmp(&b.long));

    for flag in flags {
        println!(
            "  -{}, --{:<20} {}",
            flag.short, flag.long, flag.description
        );
    }

    println!("  --version                    Show version");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn fmt_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

// ---------------------------------------------------------------------------
// Scan result
// ---------------------------------------------------------------------------

struct ScanResult {
    matches: usize,
    files: usize,
    dirs: usize,
    duration: std::time::Duration,
    paths: Vec<PathBuf>,
}

// ---------------------------------------------------------------------------
// Runtime config
// ---------------------------------------------------------------------------

struct Config {
    case_sensitive: bool,
    quiet: bool,
    all: bool,
    extension: Option<String>,
    matcher: Option<AhoCorasick>,
    limit: Option<usize>,
    threads: usize,
    collect_paths: bool,
}

// ---------------------------------------------------------------------------
// Single drive/directory scan
// ---------------------------------------------------------------------------

fn scan_dir(dir: &PathBuf, config: &Config) -> ScanResult {
    let mut builder = WalkBuilder::new(dir);
    builder
        .standard_filters(false)
        .ignore(false)
        .parents(false)
        .hidden(false)
        .follow_links(false)
        .same_file_system(false)
        .threads(config.threads);

    let walker = builder.build_parallel();

    let matches = Arc::new(AtomicUsize::new(0));
    let files = Arc::new(AtomicUsize::new(0));
    let dirs_count = Arc::new(AtomicUsize::new(0));
    let paths: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));

    let start = Instant::now();

    walker.run(|| {
        let ext = config.extension.clone();
        let all = config.all;
        let case_sensitive = config.case_sensitive;
        let quiet = config.quiet;
        let collect_paths = config.collect_paths;
        let scan_dir = dir.clone();
        let matcher = config.matcher.clone();
        let matches = Arc::clone(&matches);
        let files = Arc::clone(&files);
        let dirs_count = Arc::clone(&dirs_count);
        let paths = Arc::clone(&paths);
        let limit = config.limit;

        Box::new(move |res: Result<DirEntry, _>| -> WalkState {
            let entry = match res {
                Ok(e) => e,
                Err(_) => return WalkState::Continue,
            };

            let ft = match entry.file_type() {
                Some(ft) => ft,
                None => return WalkState::Continue,
            };

            if ft.is_dir() {
                dirs_count.fetch_add(1, Ordering::Relaxed);
                return WalkState::Continue;
            }
            if !ft.is_file() {
                return WalkState::Continue;
            }

            files.fetch_add(1, Ordering::Relaxed);

            let matched = if all {
                true
            } else if let Some(ref ext) = ext {
                entry.path().extension().is_some_and(|e| {
                    if case_sensitive {
                        e == OsStr::new(ext)
                    } else {
                        e.eq_ignore_ascii_case(OsStr::new(ext))
                    }
                })
            } else if let Some(ref m) = matcher {
                let name = entry.file_name().to_string_lossy();
                m.is_match(name.as_ref())
            } else {
                false
            };

            if matched {
                let mc = matches.fetch_add(1, Ordering::Relaxed) + 1;

                if collect_paths && let Ok(mut p) = paths.lock() {
                    p.push(entry.path().to_path_buf());
                }

                if !quiet && !all {
                    let rel = entry.path().strip_prefix(&scan_dir).unwrap_or(entry.path());
                    let disp = if rel.as_os_str().is_empty() {
                        ".".to_string()
                    } else {
                        rel.to_string_lossy().into_owned()
                    };
                    println!("{}", disp.bright_cyan());
                }

                if let Some(lim) = limit
                    && mc >= lim
                {
                    return WalkState::Quit;
                }
            }

            WalkState::Continue
        })
    });

    let collected_paths = Arc::try_unwrap(paths)
        .unwrap_or_default()
        .into_inner()
        .unwrap_or_default();

    ScanResult {
        matches: matches.load(Ordering::Relaxed),
        files: files.load(Ordering::Relaxed),
        dirs: dirs_count.load(Ordering::Relaxed),
        duration: start.elapsed(),
        paths: collected_paths,
    }
}

// ---------------------------------------------------------------------------
// Windows: enumerate all drives
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn get_all_drives() -> Vec<PathBuf> {
    ('A'..='Z')
        .map(|c| PathBuf::from(format!("{}:\\", c)))
        .filter(|p| p.exists())
        .collect()
}

// ---------------------------------------------------------------------------
// Open a file with the OS default handler
// ---------------------------------------------------------------------------

fn open_file(path: &std::path::Path) -> Result<()> {
    println!("{} {}", "Launching:".green().bold(), path.display());

    #[cfg(windows)]
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &path.to_string_lossy()])
        .spawn()?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(path).spawn()?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open").arg(path).spawn()?;

    Ok(())
}

fn prompt_and_open(paths: &[PathBuf]) -> Result<()> {
    println!(
        "{}",
        "Found more than 1 result! Pick one of the following:".yellow()
    );
    for (i, path) in paths.iter().enumerate() {
        println!("  [{}] {}", i + 1, path.display());
    }
    print!("\nEnter number to open (or q to quit): ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "q" || input == "Q" {
        return Ok(());
    }

    match input.parse::<usize>() {
        Ok(n) if n >= 1 && n <= paths.len() => {
            open_file(&paths[n - 1])?;
        }
        _ => {
            bail!("Invalid selection. Run ldx again to try.");
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let ldx_config = load_config()?;
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print_help(&ldx_config);
        return Ok(());
    }

    if args.contains("--version") {
        println!("localdex v0.0.2");
        return Ok(());
    }

    let extension: Option<String> = args
        .opt_value_from_str(["-e", "--extension"])?
        .map(|s: String| s.trim_start_matches('.').to_lowercase());

    let mut dir: PathBuf = args
        .opt_value_from_str(["--dir", "-d"])?
        .unwrap_or_else(|| ".".into());

    let case_sensitive = args.contains(["-s", "--case-sensitive"]);
    let quiet = args.contains(["-q", "--quiet"]);
    let stats = args.contains(["-S", "--stats"]);
    let all = args.contains(["-a", "--all-files"]);
    let verbose = args.contains(["-v", "--verbose"]);
    let first = args.contains(["-1", "--first"]);
    let open = args.contains(["-o", "--open"]);
    let threads: usize = args
        .opt_value_from_str(["-t", "--threads"])?
        .unwrap_or_else(num_cpus::get);
    let limit: Option<usize> = args.opt_value_from_str(["-L", "--limit"])?;

    #[cfg(windows)]
    let all_drives = args.contains(["-A", "--all-drives"]);
    #[cfg(not(windows))]
    let all_drives = false;

    if first && limit.is_some() {
        bail!("-1/--first and -L/--limit cannot be used together. Run with --help for usage.");
    }

    let limit = if first { Some(1) } else { limit };

    let pattern_opt: Option<String> = args.opt_free_from_str()?;

    let remaining = args.finish();
    if !remaining.is_empty() {
        bail!(
            "Unexpected args: {:?}. Run with --help for usage.",
            remaining
        );
    }

    if let Some(ref p) = pattern_opt
        && p.starts_with('-')
    {
        bail!("Unexpected arg: {:?}. Run with --help for usage.", p);
    }

    if all && (pattern_opt.is_some() || extension.is_some()) {
        bail!(
            "-a/--all-files cannot be combined with a pattern or -e/--extension. Run with --help for usage."
        );
    }

    if open && all {
        bail!("-o/--open cannot be combined with -a/--all-files. Run with --help for usage.");
    }

    if pattern_opt.is_some() && extension.is_some() {
        bail!(
            "Cannot use both a pattern and -e/--extension at the same time. Run with --help for usage."
        );
    }

    if !all && pattern_opt.is_none() && extension.is_none() {
        bail!(
            "Either a pattern, -e/--extension, or -a/--all-files is required. Run with --help for usage."
        );
    }

    if !all_drives {
        if dir.starts_with("~") {
            if let Some(mut home) = home_dir() {
                let rest = dir.strip_prefix("~").unwrap();
                if !rest.as_os_str().is_empty() {
                    home.push(
                        rest.strip_prefix(std::path::MAIN_SEPARATOR_STR)
                            .unwrap_or(rest),
                    );
                }
                dir = home;
            } else {
                eprintln!("~ expansion failed; using literal path.");
            }
        }

        #[cfg(windows)]
        {
            if dir.as_os_str().to_string_lossy().ends_with(':') {
                dir.push("\\");
            }
        }

        let dir = std::fs::canonicalize(&dir).unwrap_or(dir);

        #[cfg(windows)]
        let dir = {
            let s = dir.to_string_lossy();
            if let Some(stripped) = s.strip_prefix(r"\\?\") {
                PathBuf::from(stripped)
            } else {
                dir
            }
        };

        if !quiet {
            println!("Searching in: {}", dir.display());
        }

        let matcher: Option<AhoCorasick> = if !all && extension.is_none() {
            let pattern = pattern_opt.unwrap();
            Some(if case_sensitive {
                AhoCorasick::new([&pattern])?
            } else {
                AhoCorasickBuilder::new()
                    .ascii_case_insensitive(true)
                    .build([&pattern])?
            })
        } else {
            None
        };

        let config = Config {
            case_sensitive,
            quiet,
            all,
            extension,
            matcher,
            limit,
            threads,
            collect_paths: open,
        };

        let result = scan_dir(&dir, &config);
        let tc = result.files + result.dirs;

        if all {
            println!(
                "Found {} file{} in {:.3}s",
                fmt_num(result.matches),
                if result.matches == 1 { "" } else { "s" },
                result.duration.as_secs_f64()
            );
        } else {
            println!(
                "Found {} matching file{} in {:.3}s",
                fmt_num(result.matches),
                if result.matches == 1 { "" } else { "s" },
                result.duration.as_secs_f64()
            );
        }

        if stats && result.duration.as_secs_f64() > 0.0 {
            let s = result.duration.as_secs_f64();
            if verbose {
                println!(
                    "Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s | Threads: {}",
                    fmt_num(tc),
                    fmt_num(result.files),
                    fmt_num(result.dirs),
                    fmt_num((tc as f64 / s) as usize),
                    threads
                );
            } else {
                println!(
                    "Scanned {} entries | {} entries/s | Threads: {}",
                    fmt_num(tc),
                    fmt_num((tc as f64 / s) as usize),
                    threads
                );
            }
        }

        if result.matches == 0 {
            std::process::exit(1);
        }

        if open {
            match result.paths.len() {
                0 => {}
                1 => open_file(&result.paths[0])?,
                _ => prompt_and_open(&result.paths)?,
            }
        }
    } else {
        // -A / --all-drives
        #[cfg(windows)]
        {
            let drives = get_all_drives();

            let matcher: Option<AhoCorasick> = if !all && extension.is_none() {
                let pattern = pattern_opt.unwrap();
                Some(if case_sensitive {
                    AhoCorasick::new([&pattern])?
                } else {
                    AhoCorasickBuilder::new()
                        .ascii_case_insensitive(true)
                        .build([&pattern])?
                })
            } else {
                None
            };

            let config = Config {
                case_sensitive,
                quiet,
                all,
                extension,
                matcher,
                limit,
                threads,
                collect_paths: open,
            };

            let total_start = Instant::now();
            let mut total_matches = 0usize;
            let mut total_files = 0usize;
            let mut total_dirs = 0usize;

            for drive in &drives {
                if !quiet {
                    println!("Searching in: {}", drive.display());
                }

                let result = scan_dir(drive, &config);
                let tc = result.files + result.dirs;

                total_matches += result.matches;
                total_files += result.files;
                total_dirs += result.dirs;

                if all {
                    println!(
                        "  Found {} file{} in {:.3}s",
                        fmt_num(result.matches),
                        if result.matches == 1 { "" } else { "s" },
                        result.duration.as_secs_f64()
                    );
                } else {
                    println!(
                        "  Found {} matching file{} in {:.3}s",
                        fmt_num(result.matches),
                        if result.matches == 1 { "" } else { "s" },
                        result.duration.as_secs_f64()
                    );
                }

                if stats && result.duration.as_secs_f64() > 0.0 {
                    let s = result.duration.as_secs_f64();
                    if verbose {
                        println!(
                            "  Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s",
                            fmt_num(tc),
                            fmt_num(result.files),
                            fmt_num(result.dirs),
                            fmt_num((tc as f64 / s) as usize),
                        );
                    } else {
                        println!(
                            "  Scanned {} entries | {} entries/s",
                            fmt_num(tc),
                            fmt_num((tc as f64 / s) as usize),
                        );
                    }
                }
            }

            let total_dur = total_start.elapsed();
            let total_tc = total_files + total_dirs;

            println!();
            if all {
                println!(
                    "Total: {} file{} across {} drive{} in {:.3}s",
                    fmt_num(total_matches),
                    if total_matches == 1 { "" } else { "s" },
                    drives.len(),
                    if drives.len() == 1 { "" } else { "s" },
                    total_dur.as_secs_f64()
                );
            } else {
                println!(
                    "Total: {} matching file{} across {} drive{} in {:.3}s",
                    fmt_num(total_matches),
                    if total_matches == 1 { "" } else { "s" },
                    drives.len(),
                    if drives.len() == 1 { "" } else { "s" },
                    total_dur.as_secs_f64()
                );
            }

            if stats && total_dur.as_secs_f64() > 0.0 {
                let s = total_dur.as_secs_f64();
                if verbose {
                    println!(
                        "Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s | Threads: {}",
                        fmt_num(total_tc),
                        fmt_num(total_files),
                        fmt_num(total_dirs),
                        fmt_num((total_tc as f64 / s) as usize),
                        threads
                    );
                } else {
                    println!(
                        "Scanned {} entries | {} entries/s | Threads: {}",
                        fmt_num(total_tc),
                        fmt_num((total_tc as f64 / s) as usize),
                        threads
                    );
                }
            }

            if total_matches == 0 {
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
