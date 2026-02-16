mod config;
mod display;
mod launcher;
mod search;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::{Result, bail};
use dirs::home_dir;
use pico_args::Arguments;
use std::path::PathBuf;
use std::time::Instant;

use config::load_config;
use display::{fmt_num, print_help};
use launcher::{open_file, prompt_and_open};
use search::{Config, scan_dir};

#[cfg(windows)]
use search::get_all_drives;

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
        println!("localdex v0.0.3");
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
    let max_threads = num_cpus::get();
    let threads: usize = {
        let requested: Option<usize> = args.opt_value_from_str(["-t", "--threads"])?;
        match requested {
            Some(n) if n > max_threads => {
                eprintln!(
                    "Warning: {} threads requested but only {} logical cores available. Capping at {}.",
                    n, max_threads, max_threads
                );
                max_threads
            }
            Some(n) => n,
            None => max_threads,
        }
    };
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
