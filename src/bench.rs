use anyhow::Result;
use std::path::PathBuf;

use crate::bench_runner::{BenchConfig, run_benchmark};
use crate::bench_output::{save_markdown, save_csv};
use crate::config::{LdxConfig, config_path};
use chrono::Local;
use dirs::home_dir;

// ---------------------------------------------------------------------------
// Bench args
// ---------------------------------------------------------------------------

pub struct BenchArgs {
    pub threads:  usize,
    pub runs:     usize,
    pub dirs:     Vec<PathBuf>,
    pub out:      Option<PathBuf>,
    pub live:     bool,
    pub csv:      bool,
    pub edit:     bool,
}

// ---------------------------------------------------------------------------
// Parse bench-specific args (called with args AFTER "bench")
// ---------------------------------------------------------------------------

pub fn parse_bench_args(raw: &[String], config: &LdxConfig) -> Result<BenchArgs> {
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let mut threads = max_threads;
    let mut runs    = 10;
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut out:  Option<PathBuf> = None;
    let mut live  = false;
    let mut csv   = false;
    let mut edit  = false;

    // Reuse threads flag name from config
    let threads_flag = config
        .flags
        .values()
        .find(|f| f.target.as_deref() == Some("threads"))
        .map(|f| format!("-{}", f.short))
        .unwrap_or_else(|| "-t".to_string());

    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--edit"  => { edit = true; }
            "--live"  => { live = true; }
            "--csv"   => { csv  = true; }
            "--runs"  => {
                i += 1;
                runs = raw.get(i)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(runs);
            }
            "--out"   => {
                i += 1;
                out = raw.get(i).map(PathBuf::from);
            }
            "--dirs"  => {
                i += 1;
                while i < raw.len() && !raw[i].starts_with('-') {
                    dirs.push(PathBuf::from(&raw[i]));
                    i += 1;
                }
                continue;
            }
            flag if flag == threads_flag || flag == "--threads" => {
                i += 1;
                threads = raw.get(i)
                    .and_then(|v| v.parse().ok())
                    .map(|n: usize| n.min(max_threads))
                    .unwrap_or(threads);
            }
            unknown => {
                anyhow::bail!("Unknown bench flag: {:?}. Run `ldx bench --help` for usage.", unknown);
            }
        }
        i += 1;
    }

    // Always include defaults, --dirs appends to them
    let mut default_dirs = Vec::new();
    if let Some(home) = home_dir() {
        default_dirs.push(home);
    }
    default_dirs.push(PathBuf::from("/usr"));
    default_dirs.push(PathBuf::from("/"));
    default_dirs.extend(dirs);

    Ok(BenchArgs { threads, runs, dirs: default_dirs, out, live, csv, edit })
}

// ---------------------------------------------------------------------------
// run — entry point called from main.rs
// ---------------------------------------------------------------------------

pub fn run(raw: &[String], config: &LdxConfig) -> Result<()> {
    // Help
    if raw.iter().any(|a| a == "--help" || a == "-h") {
        print_bench_help();
        return Ok(());
    }

    let args = parse_bench_args(raw, config)?;

    // --edit: open bench section of config
    if args.edit {
        let path = config_path();
        println!("Opening config: {}", path.display());

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open").arg(&path).spawn()?;
        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(&path).spawn()?;
        #[cfg(windows)]
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &path.to_string_lossy()])
            .spawn()?;

        return Ok(());
    }

    let bench_config = BenchConfig {
        threads: args.threads,
        runs:    args.runs,
        dirs:    args.dirs.clone(),
        live:    args.live,
    };

    println!();
    println!("  {} ldx bench", colored::Colorize::cyan("🔍"));
    println!("  Threads : {}", args.threads);
    println!("  Runs    : {}", args.runs);
    println!("  Dirs    : {}", args.dirs.iter()
        .map(|d| d.display().to_string())
        .collect::<Vec<_>>()
        .join(", "));
    println!();

    let results = run_benchmark(&bench_config)?;

    // Output path
    let out_path = args.out.unwrap_or_else(|| {
        let ts = Local::now().format("%Y-%m-%d_%H-%M-%S");
        PathBuf::from(format!("ldx-bench-{}.md", ts))
    });

    save_markdown(&results, &bench_config, &out_path)?;
    println!("  Report saved: {}", out_path.display());

    if args.csv {
        let csv_path = out_path.with_extension("csv");
        save_csv(&results, &csv_path)?;
        println!("  CSV saved:    {}", csv_path.display());
    }

    println!();

    Ok(())
}

// ---------------------------------------------------------------------------
// Help
// ---------------------------------------------------------------------------

fn print_bench_help() {
    println!();
    println!("  {} ldx bench — benchmark ldx against your filesystem", colored::Colorize::cyan("🔍"));
    println!();
    println!("  Usage: ldx bench [options]");
    println!();
    println!("  Options:");
    println!("    -t, --threads N      Thread count (default: max)");
    println!("    --runs N             Runs per directory (default: 10)");
    println!("    --dirs d1 d2 ...     Directories to benchmark (default: $HOME)");
    println!("    --out FILE           Output file path (default: ldx-bench-<timestamp>.md)");
    println!("    --live               Print results as they run");
    println!("    --csv                Also save a CSV alongside the report");
    println!("    --edit               Open config to customize bench defaults");
    println!();
}
