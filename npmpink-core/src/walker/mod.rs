pub mod walk;

use std::path::PathBuf;
pub use walk::{walk, WalkOption};

#[derive(Debug)]
pub enum ReceiveMode {
    Buffer,
    Stream,
}

pub type ResultItem = PathBuf;
