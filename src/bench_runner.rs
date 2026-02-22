use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;

use crate::search::{Config as SearchConfig, scan_dir};

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

pub struct BenchConfig {
    pub threads: usize,
    pub runs:    usize,
    pub dirs:    Vec<PathBuf>,
    pub live:    bool,
}

// ---------------------------------------------------------------------------
// A single benchmark result for one tool + dir combination
// ---------------------------------------------------------------------------

pub struct BenchResult {
    pub tool:    String,
    pub dir:     PathBuf,
    pub avg:     f64,
    pub median:  f64,
    pub min:     f64,
    pub max:     f64,
    pub runs:    usize,
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
// Run ldx scan and return entries/s
// ---------------------------------------------------------------------------

fn run_ldx_once(dir: &PathBuf, threads: usize) -> Option<(f64, usize)> {
    let config = SearchConfig {
        case_sensitive: false,
        quiet:          true,
        all:            true,
        dirs_only:      false,
        extension:      None,
        pattern:        None,
        limit:          None,
        threads,
        collect_paths:  false,
        collect_errors: false,
        exclude:        vec![],
    };

    let result = scan_dir(dir, &config);
    let entries = result.files + result.dirs;
    let secs = result.duration.as_secs_f64();

    if secs > 0.0 && entries > 0 {
        Some((entries as f64 / secs, entries))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Run a competitor tool and return entries/s using wall time
// ---------------------------------------------------------------------------

fn run_competitor_once(tool: &str, dir: &PathBuf) -> Option<f64> {
    let start = Instant::now();

    let status = match tool {
        "fd" => std::process::Command::new("fd")
            .args(["--no-ignore", "--hidden", "."])
            .arg(dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
        "rg" => std::process::Command::new("rg")
            .args(["--files", "--no-ignore", "--hidden"])
            .arg(dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
        _ => return None,
    };

    let elapsed = start.elapsed().as_secs_f64();

    if status.map(|s| s.success()).unwrap_or(false) && elapsed > 0.0 {
        // We don't know exact entry count for competitors so we use a rough
        // estimate — just track raw time and report as relative
        Some(elapsed)
    } else {
        None
    }
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

        // ── ldx ──────────────────────────────────────────────────────────────
        let mut speeds: Vec<f64> = Vec::new();
        let mut last_entries = 0usize;

        for i in 0..config.runs {
            if !config.live {
                print!("\r  ldx  {}  [{}/{}]", dir.display(), i + 1, config.runs);
                let _ = std::io::Write::flush(&mut std::io::stdout());
            }

            if let Some((speed, entries)) = run_ldx_once(dir, config.threads) {
                speeds.push(speed);
                last_entries = entries;

                if config.live {
                    println!("    ldx  run {:>2}  {:.0} entries/s", i + 1, speed);
                }
            }
        }

        if !config.live {
            print!("\r{:<80}\r", ""); // clear line
        }

        if !speeds.is_empty() {
            let (avg, median, min, max) = calc_stats(speeds);
            if config.live {
                println!("    ldx  avg {:.0} | med {:.0} | min {:.0} | max {:.0} entries/s", avg, median, min, max);
            }
            results.push(BenchResult {
                tool: "ldx".to_string(),
                dir: dir.clone(),
                avg, median, min, max,
                runs: config.runs,
                entries: last_entries,
            });
        }

        // ── Competitors ───────────────────────────────────────────────────────
        for tool in &competitors {
            let mut times: Vec<f64> = Vec::new();

            for i in 0..config.runs {
                if !config.live {
                    print!("\r  {}  {}  [{}/{}]", tool, dir.display(), i + 1, config.runs);
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                }

                if let Some(elapsed) = run_competitor_once(tool, dir) {
                    times.push(elapsed);

                    if config.live {
                        println!("    {}  run {:>2}  {:.3}s", tool, i + 1, elapsed);
                    }
                }
            }

            if !config.live {
                print!("\r{:<80}\r", "");
            }

            if !times.is_empty() {
                // Convert elapsed seconds → entries/s using ldx's entry count as reference
                let entries = last_entries as f64;
                let speeds: Vec<f64> = times.iter().map(|t| if *t > 0.0 { entries / t } else { 0.0 }).collect();
                let (avg, median, min, max) = calc_stats(speeds);
                if config.live {
                    println!("    {}  avg {:.0} | med {:.0} | min {:.0} | max {:.0} entries/s", tool, avg, median, min, max);
                }
                results.push(BenchResult {
                    tool: tool.clone(),
                    dir: dir.clone(),
                    avg, median, min, max,
                    runs: config.runs,
                    entries: last_entries,
                });
            }
        }

        if config.live { println!(); }
        println!("  ✓ {}", dir.display());
    }

    println!();
    Ok(results)
}
