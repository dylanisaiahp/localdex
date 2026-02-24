use std::path::PathBuf;
use std::sync::mpsc;

use ignore::WalkBuilder;
use parex::Source;
use parex::engine::WalkConfig;
use parex::{Entry, EntryKind, ParexError};

// Batch size for channel sends — reduces contention dramatically
const BATCH_SIZE: usize = 128;

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

        // Batched channel — sends Vec<Result<Entry>> instead of one at a time
        let (tx, rx) = mpsc::channel::<Vec<Result<Entry, ParexError>>>();

        std::thread::spawn(move || {
            builder.build_parallel().run(|| {
                let tx = tx.clone();
                let exclude = exclude.clone();

                // Flush-on-drop wrapper — ensures partial batch is sent when worker closes
                struct BatchFlusher {
                    batch: Vec<Result<Entry, ParexError>>,
                    tx: mpsc::Sender<Vec<Result<Entry, ParexError>>>,
                }
                impl Drop for BatchFlusher {
                    fn drop(&mut self) {
                        if !self.batch.is_empty() {
                            let _ = self.tx.send(std::mem::take(&mut self.batch));
                        }
                    }
                }

                let mut flusher = BatchFlusher {
                    batch: Vec::with_capacity(BATCH_SIZE),
                    tx: tx.clone(),
                };

                Box::new(move |res| {
                    use ignore::WalkState;

                    // Always flush errors immediately
                    if let Err(err) = res {
                        if !flusher.batch.is_empty() {
                            let _ = tx.send(std::mem::take(&mut flusher.batch));
                            flusher.batch = Vec::with_capacity(BATCH_SIZE);
                        }
                        let _ = tx.send(vec![Err(map_ignore_error(err))]);
                        return WalkState::Continue;
                    }

                    let entry = res.unwrap();

                    // Skip root entry itself
                    if entry.depth() == 0 {
                        return WalkState::Continue;
                    }

                    // Safely extract FileType
                    let file_type = entry.file_type();
                    let is_dir = file_type.map(|ft| ft.is_dir()).unwrap_or(false);
                    let is_file = file_type.map(|ft| ft.is_file()).unwrap_or(false);
                    let is_symlink = file_type.map(|ft| ft.is_symlink()).unwrap_or(false);

                    // Forward unreadable directories as errors
                    if is_dir && let Err(e) = std::fs::read_dir(entry.path()) {
                            let pe = if e.kind() == std::io::ErrorKind::PermissionDenied {
                                ParexError::PermissionDenied(entry.path().to_path_buf())
                            } else {
                                ParexError::Io {
                                    path: entry.path().to_path_buf(),
                                    source: e,
                                }
                            };
                            if !flusher.batch.is_empty() {
                                let _ = tx.send(std::mem::take(&mut flusher.batch));
                                flusher.batch = Vec::with_capacity(BATCH_SIZE);
                            }
                            let _ = tx.send(vec![Err(pe)]);
                            return WalkState::Continue;
                        }

                    // Skip excluded directories
                    if is_dir && exclude.contains(&entry.file_name().to_string_lossy().to_string()) {
                        return WalkState::Skip;
                    }

                    // dirs_only mode
                    if dirs_only && is_file {
                        return WalkState::Continue;
                    }

                    // Build EntryKind
                    let kind = if is_dir {
                        EntryKind::Dir
                    } else if is_symlink {
                        EntryKind::Symlink
                    } else {
                        EntryKind::File
                    };

                    flusher.batch.push(Ok(Entry {
                        path: entry.path().to_path_buf(),
                        kind,
                        depth: entry.depth(),
                        metadata: None,
                    }));

                    if flusher.batch.len() >= BATCH_SIZE {
                        let _ = tx.send(std::mem::take(&mut flusher.batch));
                        flusher.batch = Vec::with_capacity(BATCH_SIZE);
                    }

                    WalkState::Continue
                })
            });
        });

        // Flatten batches back into individual items for the parex engine
        Box::new(rx.into_iter().flatten())
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
