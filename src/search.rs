use std::path::PathBuf;
use std::time::Duration;

use parex::Matcher;

use crate::source::DirectorySource;

// ---------------------------------------------------------------------------
// Scan result
// ---------------------------------------------------------------------------

pub struct ScanResult {
    pub matches: usize,
    pub files: usize,
    pub dirs: usize,
    pub duration: Duration,
    pub paths: Vec<PathBuf>,
    pub errors: Vec<parex::ParexError>,
}

// ---------------------------------------------------------------------------
// Search config
// ---------------------------------------------------------------------------

pub struct Config {
    pub case_sensitive: bool,
    pub quiet: bool,
    pub all: bool,
    pub dirs_only: bool,
    pub extension: Option<String>,
    pub pattern: Option<String>,
    pub limit: Option<usize>,
    pub threads: usize,
    pub collect_paths: bool,
    pub collect_errors: bool,
    pub exclude: Vec<String>,
}

// ---------------------------------------------------------------------------
// Matchers
// ---------------------------------------------------------------------------

/// Matches files by name substring (case-insensitive or sensitive).
struct NameMatcher {
    pattern: String,
    case_sensitive: bool,
}

impl Matcher for NameMatcher {
    fn is_match(&self, entry: &parex::Entry) -> bool {
        let name = entry.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if self.case_sensitive {
            name.contains(&self.pattern)
        } else {
            name.to_lowercase().contains(&self.pattern.to_lowercase())
        }
    }
}

/// Matches files by extension.
struct ExtMatcher {
    ext: String,
    case_sensitive: bool,
}

impl Matcher for ExtMatcher {
    fn is_match(&self, entry: &parex::Entry) -> bool {
        entry.path.extension().is_some_and(|e| {
            if self.case_sensitive {
                e == std::ffi::OsStr::new(&self.ext)
            } else {
                e.eq_ignore_ascii_case(std::ffi::OsStr::new(&self.ext))
            }
        })
    }
}

/// Matches everything — used for -a/--all-files.
struct AllMatcher;

impl Matcher for AllMatcher {
    fn is_match(&self, entry: &parex::Entry) -> bool {
        matches!(entry.kind, parex::EntryKind::File)
    }
}

/// Matches directories only.
struct DirMatcher {
    pattern: Option<String>,
    case_sensitive: bool,
}

impl Matcher for DirMatcher {
    fn is_match(&self, entry: &parex::Entry) -> bool {
        if !matches!(entry.kind, parex::EntryKind::Dir) {
            return false;
        }
        match &self.pattern {
            None => true,
            Some(pat) => {
                let name = entry.path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if self.case_sensitive {
                    name.contains(pat)
                } else {
                    name.to_lowercase().contains(&pat.to_lowercase())
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// scan_dir — thin wrapper around parex::search()
// ---------------------------------------------------------------------------

pub fn scan_dir(dir: &PathBuf, config: &Config) -> ScanResult {
    let source = DirectorySource::new(dir)
        .exclude(config.exclude.clone())
        .dirs_only(config.dirs_only)
        .follow_links(false);

    let mut builder = parex::search()
        .source(source)
        .threads(config.threads)
        .collect_paths(config.collect_paths)
        .collect_errors(config.collect_errors);

    if let Some(lim) = config.limit {
        builder = builder.limit(lim);
    }

    // Wire up the right matcher
    let result = if config.all {
        builder.with_matcher(AllMatcher).run()
    } else if config.dirs_only {
        builder
            .with_matcher(DirMatcher {
                pattern: config.pattern.clone(),
                case_sensitive: config.case_sensitive,
            })
            .run()
    } else if let Some(ext) = &config.extension {
        builder
            .with_matcher(ExtMatcher {
                ext: ext.clone(),
                case_sensitive: config.case_sensitive,
            })
            .run()
    } else {
        builder
            .with_matcher(NameMatcher {
                pattern: config.pattern.clone().unwrap_or_default(),
                case_sensitive: config.case_sensitive,
            })
            .run()
    };

    let result = result.expect("parex search failed");

    // Print matches as we go — parex doesn't handle output, we do
    if !config.quiet && !config.all {
        for path in &result.paths {
            let rel = path.strip_prefix(dir).unwrap_or(path);
            let disp = if rel.as_os_str().is_empty() {
                ".".to_string()
            } else {
                rel.to_string_lossy().into_owned()
            };
            println!("{}", colored::Colorize::bright_cyan(disp.as_str()));
        }
    }

    ScanResult {
        matches: result.matches,
        files: result.stats.files,
        dirs: result.stats.dirs,
        duration: result.stats.duration,
        paths: result.paths,
        errors: result.errors,
    }
}
