mod config;
mod display;
mod flags;
mod launcher;
mod search;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::Result;
use dirs::home_dir;
use std::path::PathBuf;
use std::time::Instant;

use config::{check_config, config_path, load_config, reset_config, sync_config};
use display::{fmt_num, print_help, print_result, print_stats};
use flags::{ParsedFlags, parse_args};
use launcher::{open_file, prompt_and_open};
use search::{Config, ScanResult, scan_dir};

#[cfg(windows)]
use search::get_all_drives;

// ---------------------------------------------------------------------------
// Build AhoCorasick matcher from parsed flags
// ---------------------------------------------------------------------------

fn build_matcher(f: &ParsedFlags) -> Result<Option<AhoCorasick>> {
    if f.all || f.extension.is_some() {
        return Ok(None);
    }
    let pattern = f.pattern.clone().unwrap();
    Ok(Some(if f.case_sensitive {
        AhoCorasick::new([&pattern])?
    } else {
        AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .build([&pattern])?
    }))
}

// ---------------------------------------------------------------------------
// Build search::Config from parsed flags
// ---------------------------------------------------------------------------

fn build_search_config(
    f: &ParsedFlags,
    matcher: Option<AhoCorasick>,
    collect_paths: bool,
) -> Config {
    Config {
        case_sensitive: f.case_sensitive,
        quiet: f.quiet,
        all: f.all,
        dirs_only: f.dirs_only,
        extension: f.extension.clone(),
        matcher,
        limit: f.limit,
        threads: f.threads,
        collect_paths,
        exclude: f.exclude.clone(),
    }
}

// ---------------------------------------------------------------------------
// Resolve and canonicalize the search directory
// ---------------------------------------------------------------------------

fn resolve_dir(dir: PathBuf) -> PathBuf {
    let mut dir = dir;

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
    if dir.as_os_str().to_string_lossy().ends_with(':') {
        dir.push("\\");
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

    dir
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let ldx_config = load_config()?;
    let f = parse_args(&ldx_config)?;

    // ── Management flags ──────────────────────────────────────────────────────

    if f.show_help {
        print_help(&ldx_config);
        return Ok(());
    }

    if f.show_version {
        println!("localdex v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if f.show_config {
        println!("Config: {}", config_path().display());
        return Ok(());
    }

    if f.edit_config {
        let path = config_path();
        println!("Opening config: {}", path.display());

        #[cfg(windows)]
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &path.to_string_lossy()])
            .spawn()?;

        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(&path).spawn()?;

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open").arg(&path).spawn()?;

        return Ok(());
    }

    if f.check_config {
        check_config(&ldx_config);
        return Ok(());
    }

    if f.sync_config {
        sync_config()?;
        return Ok(());
    }

    if f.reset_config {
        reset_config()?;
        return Ok(());
    }

    // ── Search ────────────────────────────────────────────────────────────────

    let matcher = build_matcher(&f)?;

    if !f.all_drives {
        let dir = resolve_dir(f.dir.clone());

        if !f.quiet {
            println!("Searching in: {}", dir.display());
        }

        let config = build_search_config(&f, matcher, f.open || f.where_mode);
        let result = scan_dir(&dir, &config);
        let reported_matches = clamp_matches(&result, f.limit);

        print_result(&result, reported_matches, &f, "");
        print_stats(&result, &f, "");

        if reported_matches == 0 {
            std::process::exit(1);
        }

        if f.open {
            match result.paths.len() {
                0 => {}
                1 => open_file(&result.paths[0])?,
                _ => prompt_and_open(&result.paths)?,
            }
        }

        if f.where_mode {
            match result.paths.len() {
                0 => {}
                1 => {
                    let path = &result.paths[0];
                    let dir_path = if f.dirs_only {
                        path.to_string_lossy().into_owned()
                    } else {
                        path.parent()
                            .map(|p| p.to_string_lossy().into_owned())
                            .unwrap_or_else(|| path.to_string_lossy().into_owned())
                    };
                    println!("  → cd {}", dir_path);
                }
                _ => println!("  → Multiple results found, use -1 to get a single match"),
            }
        }
    } else {
        #[cfg(windows)]
        {
            let drives = get_all_drives();
            let config = build_search_config(&f, matcher, f.open);
            let total_start = Instant::now();
            let mut total_matches = 0usize;
            let mut total_files = 0usize;
            let mut total_dirs = 0usize;

            for drive in &drives {
                if !f.quiet {
                    println!("Searching in: {}", drive.display());
                }
                let result = scan_dir(drive, &config);
                total_matches += result.matches;
                total_files += result.files;
                total_dirs += result.dirs;

                print_result(&result, result.matches, &f, "  ");
                print_stats(&result, &f, "  ");
            }

            let total_dur = total_start.elapsed();
            let total_tc = total_files + total_dirs;
            let s = total_dur.as_secs_f64();

            println!();
            if f.all {
                println!(
                    "Total: {} file{} across {} drive{} in {:.3}s",
                    fmt_num(total_matches),
                    if total_matches == 1 { "" } else { "s" },
                    drives.len(),
                    if drives.len() == 1 { "" } else { "s" },
                    s
                );
            } else {
                println!(
                    "Total: {} matching file{} across {} drive{} in {:.3}s",
                    fmt_num(total_matches),
                    if total_matches == 1 { "" } else { "s" },
                    drives.len(),
                    if drives.len() == 1 { "" } else { "s" },
                    s
                );
            }

            if f.stats && s > 0.0 {
                if f.verbose {
                    println!(
                        "Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s | Threads: {}",
                        fmt_num(total_tc),
                        fmt_num(total_files),
                        fmt_num(total_dirs),
                        fmt_num((total_tc as f64 / s) as usize),
                        f.threads
                    );
                } else {
                    println!(
                        "Scanned {} entries | {} entries/s | Threads: {}",
                        fmt_num(total_tc),
                        fmt_num((total_tc as f64 / s) as usize),
                        f.threads
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

// ---------------------------------------------------------------------------
// Clamp reported matches to limit (atomic counter can overshoot)
// ---------------------------------------------------------------------------

fn clamp_matches(result: &ScanResult, limit: Option<usize>) -> usize {
    match limit {
        Some(lim) => result.matches.min(lim),
        None => result.matches,
    }
}
