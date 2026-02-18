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

use config::load_config;
use display::{fmt_num, print_help};
use flags::parse_args;
use launcher::{open_file, prompt_and_open};
use search::{Config, scan_dir};

#[cfg(windows)]
use search::get_all_drives;

fn main() -> Result<()> {
    let ldx_config = load_config()?;

    // Get config path for --config and --edit flags
    let config_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("config.toml");

    let f = parse_args(&ldx_config)?;

    if f.show_help {
        print_help(&ldx_config);
        return Ok(());
    }

    if f.show_version {
        println!("localdex v0.0.6");
        return Ok(());
    }

    if f.show_config {
        println!("Config: {}", config_path.display());
        return Ok(());
    }

    if f.edit_config {
        println!("Opening config: {}", config_path.display());

        #[cfg(windows)]
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &config_path.to_string_lossy()])
            .spawn()?;

        #[cfg(target_os = "macos")]
        std::process::Command::new("open")
            .arg(&config_path)
            .spawn()?;

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(&config_path)
            .spawn()?;

        return Ok(());
    }

    let mut dir = f.dir.clone();

    if !f.all_drives {
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

        if !f.quiet {
            println!("Searching in: {}", dir.display());
        }

        let matcher: Option<AhoCorasick> = if !f.all && f.extension.is_none() {
            let pattern = f.pattern.clone().unwrap();
            Some(if f.case_sensitive {
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
            case_sensitive: f.case_sensitive,
            quiet: f.quiet,
            all: f.all,
            dirs_only: f.dirs_only,
            extension: f.extension.clone(),
            matcher,
            limit: f.limit,
            threads: f.threads,
            collect_paths: f.open || f.where_mode,
            exclude: f.exclude.clone(),
        };

        let result = scan_dir(&dir, &config);
        let tc = result.files + result.dirs;

        if f.all {
            println!(
                "Found {} file{} in {:.3}s",
                fmt_num(result.matches),
                if result.matches == 1 { "" } else { "s" },
                result.duration.as_secs_f64()
            );
        } else if f.dirs_only {
            println!(
                "Found {} matching director{} in {:.3}s",
                fmt_num(result.matches),
                if result.matches == 1 { "y" } else { "ies" },
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

        if f.stats && result.duration.as_secs_f64() > 0.0 {
            let s = result.duration.as_secs_f64();
            if f.verbose {
                println!(
                    "Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s | Threads: {}",
                    fmt_num(tc),
                    fmt_num(result.files),
                    fmt_num(result.dirs),
                    fmt_num((tc as f64 / s) as usize),
                    f.threads
                );
            } else {
                println!(
                    "Scanned {} entries | {} entries/s | Threads: {}",
                    fmt_num(tc),
                    fmt_num((tc as f64 / s) as usize),
                    f.threads
                );
            }
        }

        if result.matches == 0 {
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
                _ => {
                    println!("  → Multiple results found, use -1 to get a single match");
                }
            }
        }
    } else {
        #[cfg(windows)]
        {
            let drives = get_all_drives();

            let matcher: Option<AhoCorasick> = if !f.all && f.extension.is_none() {
                let pattern = f.pattern.clone().unwrap();
                Some(if f.case_sensitive {
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
                case_sensitive: f.case_sensitive,
                quiet: f.quiet,
                all: f.all,
                dirs_only: f.dirs_only,
                extension: f.extension.clone(),
                matcher,
                limit: f.limit,
                threads: f.threads,
                collect_paths: f.open,
                exclude: f.exclude.clone(),
            };

            let total_start = Instant::now();
            let mut total_matches = 0usize;
            let mut total_files = 0usize;
            let mut total_dirs = 0usize;

            for drive in &drives {
                if !f.quiet {
                    println!("Searching in: {}", drive.display());
                }

                let result = scan_dir(drive, &config);
                let tc = result.files + result.dirs;

                total_matches += result.matches;
                total_files += result.files;
                total_dirs += result.dirs;

                if f.all {
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

                if f.stats && result.duration.as_secs_f64() > 0.0 {
                    let s = result.duration.as_secs_f64();
                    if f.verbose {
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
            if f.all {
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

            if f.stats && total_dur.as_secs_f64() > 0.0 {
                let s = total_dur.as_secs_f64();
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
