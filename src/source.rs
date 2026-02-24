use std::path::PathBuf;
use std::sync::mpsc;

use parawalk::{EntryKind as WalkKind, EntryRef, WalkConfig as ParaConfig};
use parex::Source;
use parex::engine::WalkConfig;
use parex::{Entry, EntryKind, ParexError};

const BATCH_SIZE: usize = 128;

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
        let walk_config = ParaConfig {
            threads: config.threads,
            max_depth: config.max_depth,
            follow_links: self.follow_links,
        };

        let root = self.root.clone();
        let exclude = self.exclude.clone();
        let dirs_only = self.dirs_only;

        let (tx, rx) = mpsc::channel::<Vec<Entry>>();
        let tx_visitor = tx.clone();

        std::thread::spawn(move || {
            parawalk::walk(
                root,
                walk_config,
                Some(move |entry: &EntryRef<'_>| {
                    let name = entry.name.to_string_lossy();
                    if entry.kind == WalkKind::Dir && exclude.contains(&name.to_string()) {
                        return false;
                    }
                    if dirs_only && entry.kind == WalkKind::File {
                        return false;
                    }
                    true
                }),
                move || {
                    // Each thread gets its own batch — no locking needed
                    let tx = tx_visitor.clone();
                    let mut batch: Vec<Entry> = Vec::with_capacity(BATCH_SIZE);

                    move |walked: parawalk::Entry| {
                        let kind = match walked.kind {
                            WalkKind::Dir => EntryKind::Dir,
                            WalkKind::Symlink => EntryKind::Symlink,
                            WalkKind::File => EntryKind::File,
                            WalkKind::Other => return,
                        };

                        batch.push(Entry {
                            path: walked.path,
                            kind,
                            depth: walked.depth,
                            metadata: None,
                        });

                        if batch.len() >= BATCH_SIZE {
                            let _ = tx.send(std::mem::take(&mut batch));
                            batch = Vec::with_capacity(BATCH_SIZE);
                        }
                    }
                },
            );
        });

        Box::new(rx.into_iter().flatten().map(Ok))
    }
}

#[cfg(windows)]
pub fn get_all_drives() -> Vec<PathBuf> {
    ('A'..='Z')
        .map(|c| PathBuf::from(format!("{}:\\", c)))
        .filter(|p| p.exists())
        .collect()
}
