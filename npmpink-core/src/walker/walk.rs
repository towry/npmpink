use anyhow::Result;
use ignore::{WalkBuilder, WalkParallel, WalkState};
use regex::bytes::Regex;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;

#[derive(Debug, Clone)]
pub struct WalkOption {
    /// How many threads to run.
    pub threads: usize,
    /// Math depth to iterate the directories.
    pub max_depth: usize,
    pub patterns: Vec<Regex>,
}

impl WalkOption {
    pub fn new(patterns: Vec<Regex>) -> WalkOption {
        WalkOption {
            patterns,
            ..Default::default()
        }
    }
    pub fn max_depth(mut self, size: usize) -> Self {
        self.max_depth = size;
        self
    }
    pub fn threads(mut self, size: usize) -> Self {
        self.threads = size;
        self
    }
}

impl Default for WalkOption {
    fn default() -> WalkOption {
        WalkOption {
            threads: 5,
            max_depth: 10,
            patterns: Vec::new(),
        }
    }
}

/// Create a file tree walker
pub fn create_concurrent_walker(
    paths: &[impl AsRef<Path>],
    option: Option<WalkOption>,
) -> (WalkParallel, WalkOption) {
    let option = option.unwrap_or_default();

    let first_path = &paths[0];
    let mut walker = WalkBuilder::new(first_path);
    walker
        .follow_links(false)
        // .max_filesize(Some(1024 * 1024 * 30))
        .max_depth(Some(option.max_depth))
        .threads(option.threads);

    // add paths to search
    paths.iter().skip(1).for_each(|p| {
        walker.add(p);
    });

    (walker.build_parallel(), option)
}

pub fn walk(paths: &[impl AsRef<Path>], option: Option<WalkOption>) -> Result<Vec<PathBuf>> {
    let (walker, option) = create_concurrent_walker(paths, option);

    let patterns = &option.patterns;
    let mut buffers = Vec::<PathBuf>::new();

    let (tx, rx) = channel::<PathBuf>();

    walker.run(|| {
        Box::new(|result| {
            if result.is_err() {
                return WalkState::Skip;
            }
            let entry = result.unwrap();
            // skip root.
            if entry.depth() == 0 {
                return WalkState::Continue;
            }

            let entry_path = entry.path();
            if !is_match_pattern(entry_path, patterns.as_slice()) {
                return WalkState::Continue;
            }

            tx.send(entry_path.to_path_buf()).unwrap();

            WalkState::Continue
        })
    });

    drop(tx);

    while let Ok(re) = rx.recv() {
        buffers.push(re);
    }

    Ok(buffers)
}

fn is_match_pattern(path: &Path, pattern: &[Regex]) -> bool {
    let to_match: Cow<[u8]> = {
        match path.to_string_lossy() {
            Cow::Owned(string) => Cow::Owned(string.into_bytes()),
            Cow::Borrowed(string) => Cow::Borrowed(string.as_bytes()),
        }
    };

    pattern.iter().all(|pat| pat.is_match(&to_match))
}
