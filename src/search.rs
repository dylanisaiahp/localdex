use aho_corasick::AhoCorasick;
use colored::Colorize;
use ignore::{DirEntry, WalkBuilder, WalkState};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// ---------------------------------------------------------------------------
// Runtime config
// ---------------------------------------------------------------------------

pub struct Config {
    pub case_sensitive: bool,
    pub quiet: bool,
    pub all: bool,
    pub extension: Option<String>,
    pub matcher: Option<AhoCorasick>,
    pub limit: Option<usize>,
    pub threads: usize,
    pub collect_paths: bool,
}

// ---------------------------------------------------------------------------
// Scan result
// ---------------------------------------------------------------------------

pub struct ScanResult {
    pub matches: usize,
    pub files: usize,
    pub dirs: usize,
    pub duration: std::time::Duration,
    pub paths: Vec<PathBuf>,
}

// ---------------------------------------------------------------------------
// Single drive/directory scan
// ---------------------------------------------------------------------------

pub fn scan_dir(dir: &PathBuf, config: &Config) -> ScanResult {
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

                if collect_paths
                    && let Ok(mut p) = paths.lock() {
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
                    && mc >= lim {
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
pub fn get_all_drives() -> Vec<PathBuf> {
    ('A'..='Z')
        .map(|c| PathBuf::from(format!("{}:\\", c)))
        .filter(|p| p.exists())
        .collect()
}
