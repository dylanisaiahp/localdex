use std::path::PathBuf;
use std::sync::mpsc;

use ignore::WalkBuilder;
use parex::Source;
use parex::engine::WalkConfig;
use parex::{Entry, EntryKind, ParexError};

// ---------------------------------------------------------------------------
// DirectorySource
// ---------------------------------------------------------------------------

pub struct DirectorySource {
    pub root: PathBuf,
    pub exclude: Vec<String>,
    pub dirs_only: bool,
    pub follow_links: bool,
}

impl DirectorySource {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            exclude: vec![],
            dirs_only: false,
            follow_links: false,
        }
    }

    pub fn exclude(mut self, dirs: Vec<String>) -> Self {
        self.exclude = dirs;
        self
    }

    pub fn dirs_only(mut self, yes: bool) -> Self {
        self.dirs_only = yes;
        self
    }

    pub fn follow_links(mut self, yes: bool) -> Self {
        self.follow_links = yes;
        self
    }
}

impl Source for DirectorySource {
    fn walk(&self, config: &WalkConfig) -> Box<dyn Iterator<Item = Result<Entry, ParexError>>> {
        let mut builder = WalkBuilder::new(&self.root);
        builder
            .standard_filters(false)
            .ignore(false)
            .parents(false)
            .hidden(false)
            .follow_links(self.follow_links)
            .same_file_system(false)
            .threads(config.threads);

        if let Some(depth) = config.max_depth {
            builder.max_depth(Some(depth));
        }

        let exclude = self.exclude.clone();
        let dirs_only = self.dirs_only;
        let walker = builder.build_parallel();

        // Channel to stream entries from the parallel walker to the iterator
        let (tx, rx) = mpsc::channel::<Result<Entry, ParexError>>();

        std::thread::spawn(move || {
            walker.run(|| {
                let tx = tx.clone();
                let exclude = exclude.clone();

                Box::new(move |res| {
                    use ignore::WalkState;

                    let entry = match res {
                        Err(e) => {
                            let _ = tx.send(Err(map_ignore_error(e)));
                            return WalkState::Continue;
                        }
                        Ok(e) => e,
                    };

                    // Skip root itself
                    if entry.depth() == 0 {
                        return WalkState::Continue;
                    }

                    let ft = match entry.file_type() {
                        Some(ft) => ft,
                        None => return WalkState::Continue,
                    };

                    // Skip excluded directories
                    if ft.is_dir() {
                        let name = entry.file_name().to_string_lossy();
                        if exclude.iter().any(|ex| name.as_ref() == ex.as_str()) {
                            return WalkState::Skip;
                        }
                    }

                    // In dirs_only mode, skip files entirely
                    if dirs_only && ft.is_file() {
                        return WalkState::Continue;
                    }

                    let kind = if ft.is_dir() {
                        EntryKind::Dir
                    } else if ft.is_symlink() {
                        EntryKind::Symlink
                    } else {
                        EntryKind::File
                    };

                    let e = Entry {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        path: entry.path().to_path_buf(),
                        kind,
                        depth: entry.depth(),
                        metadata: None,
                    };

                    let _ = tx.send(Ok(e));
                    WalkState::Continue
                })
            });
            // tx dropped here — rx iterator ends naturally
        });

        Box::new(rx.into_iter())
    }
}

// ---------------------------------------------------------------------------
// Map ignore::Error → ParexError
// ---------------------------------------------------------------------------

fn map_ignore_error(e: ignore::Error) -> ParexError {
    match e {
        ignore::Error::WithPath { path, err } => match *err {
            ignore::Error::Io(io_err) => {
                if io_err.kind() == std::io::ErrorKind::PermissionDenied {
                    ParexError::PermissionDenied(path)
                } else {
                    ParexError::Io {
                        path,
                        source: io_err,
                    }
                }
            }
            _ => ParexError::source_err(err),
        },
        ignore::Error::Loop { child, .. } => ParexError::SymlinkLoop(child),
        ignore::Error::Io(io_err) => ParexError::Io {
            path: PathBuf::new(),
            source: io_err,
        },
        other => ParexError::source_err(other),
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
