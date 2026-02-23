use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;

use crate::search::{Config as SearchConfig, scan_dir};

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

pub struct BenchConfig {
    pub threads: usize,
    pub runs: usize,
    pub dirs: Vec<PathBuf>,
    pub live: bool,
}

// ---------------------------------------------------------------------------
// A single benchmark result for one tool + dir combination
// ---------------------------------------------------------------------------

pub struct BenchResult {
    pub tool: String,
    pub dir: PathBuf,
    pub avg: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub runs: usize,
    pub entries: usize,
}

// ---------------------------------------------------------------------------
// Detected competitor tools
// ---------------------------------------------------------------------------

fn detect_competitors() -> Vec<String> {
    let candidates = ["fd", "rg"];
    candidates
        .iter()
        .filter(|&&cmd| which(cmd))
        .map(|s| s.to_string())
        .collect()
}

fn which(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Stats helpers
// ---------------------------------------------------------------------------

fn calc_stats(mut samples: Vec<f64>) -> (f64, f64, f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len() as f64;
    let avg = samples.iter().sum::<f64>() / n;
    let median = if samples.len().is_multiple_of(2) {
        (samples[samples.len() / 2 - 1] + samples[samples.len() / 2]) / 2.0
    } else {
        samples[samples.len() / 2]
    };
    let min = samples.first().copied().unwrap_or(0.0);
    let max = samples.last().copied().unwrap_or(0.0);
    (avg, median, min, max)
}

fn progress(live: bool, msg: &str) {
    if !live {
        print!("\r  {:<78}", msg);
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}

fn clear_progress(live: bool) {
    if !live {
        print!("\r{:<80}\r", "");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}

// ---------------------------------------------------------------------------
// Run ldx — uses internal scan_dir directly
// ---------------------------------------------------------------------------

fn bench_ldx(dir: &PathBuf, config: &BenchConfig) -> Option<BenchResult> {
    let search_config = SearchConfig {
        case_sensitive: false,
        quiet: true,
        all: true,
        dirs_only: false,
        extension: None,
        pattern: None,
        limit: None,
        threads: config.threads,
        collect_paths: false,
        collect_errors: false,
        exclude: vec![],
    };

    let mut speeds = Vec::new();
    let mut entries = 0usize;

    for i in 0..config.runs {
        progress(config.live, &format!("ldx  {} [{}/{}]", dir.display(), i + 1, config.runs));

        let result = scan_dir(dir, &search_config);
        let e = result.files + result.dirs;
        let secs = result.duration.as_secs_f64();

        if secs > 0.0 && e > 0 {
            let speed = e as f64 / secs;
            speeds.push(speed);
            entries = e;
            if config.live {
                println!("    ldx  run {:>2}  {} entries/s", i + 1, fmt_speed(speed));
            }
        }
    }

    clear_progress(config.live);

    if speeds.is_empty() {
        return None;
    }

    let (avg, median, min, max) = calc_stats(speeds);
    if config.live {
        println!(
            "    ldx  avg {} | med {} | min {} | max {} entries/s",
            fmt_speed(avg), fmt_speed(median), fmt_speed(min), fmt_speed(max)
        );
    }

    Some(BenchResult {
        tool: "ldx".to_string(),
        dir: dir.clone(),
        avg, median, min, max,
        runs: config.runs,
        entries,
    })
}

// ---------------------------------------------------------------------------
// Run a competitor — captures output to count real entries
// ---------------------------------------------------------------------------

fn bench_competitor(tool: &str, dir: &PathBuf, config: &BenchConfig) -> Option<BenchResult> {
    let mut speeds = Vec::new();
    let mut entries = 0usize;

    for i in 0..config.runs {
        progress(config.live, &format!("{}  {} [{}/{}]", tool, dir.display(), i + 1, config.runs));

        let start = Instant::now();

        let output = match tool {
            "fd" => std::process::Command::new("fd")
                .args(["--no-ignore", "--hidden", "."])
                .arg(dir)
                .stderr(std::process::Stdio::null())
                .output(),
            "rg" => std::process::Command::new("rg")
                .args(["--files", "--no-ignore", "--hidden"])
                .arg(dir)
                .stderr(std::process::Stdio::null())
                .output(),
            _ => return None,
        };

        let elapsed = start.elapsed().as_secs_f64();

        if let Ok(out) = output {
            let e = out.stdout.split(|&b| b == b'\n').filter(|l| !l.is_empty()).count();
            if elapsed > 0.0 && e > 0 {
                let speed = e as f64 / elapsed;
                speeds.push(speed);
                entries = e;
                if config.live {
                    println!("    {}  run {:>2}  {} entries/s  ({} entries)", tool, i + 1, fmt_speed(speed), e);
                }
            }
        }
    }

    clear_progress(config.live);

    if speeds.is_empty() {
        return None;
    }

    let (avg, median, min, max) = calc_stats(speeds);
    if config.live {
        println!(
            "    {}  avg {} | med {} | min {} | max {} entries/s",
            tool, fmt_speed(avg), fmt_speed(median), fmt_speed(min), fmt_speed(max)
        );
    }

    Some(BenchResult {
        tool: tool.to_string(),
        dir: dir.clone(),
        avg, median, min, max,
        runs: config.runs,
        entries,
    })
}

// ---------------------------------------------------------------------------
// Format entries/s with commas
// ---------------------------------------------------------------------------

fn fmt_speed(n: f64) -> String {
    let n = n as u64;
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}

// ---------------------------------------------------------------------------
// run_benchmark
// ---------------------------------------------------------------------------

pub fn run_benchmark(config: &BenchConfig) -> Result<Vec<BenchResult>> {
    let competitors = detect_competitors();
    if !competitors.is_empty() {
        println!("  Competitors detected: {}", competitors.join(", "));
    }
    println!();

    let mut results = Vec::new();

    for dir in &config.dirs {
        if config.live {
            println!("  📂 {}", dir.display());
        }

        if let Some(r) = bench_ldx(dir, config) {
            results.push(r);
        }

        for tool in &competitors {
            if let Some(r) = bench_competitor(tool, dir, config) {
                results.push(r);
            }
        }

        if config.live { println!(); }
        println!("  ✓ {}", dir.display());
    }

    println!();
    Ok(results)
}
