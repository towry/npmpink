use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Source {
    /// Absolute path
    pub path: PathBuf,
    pub id: String,
}

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

fn hash_path_string(str: String) -> String {
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish().to_string()
}

impl Source {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let pathbuf = path.as_ref().to_path_buf();
        // cli module need to known and handle the absolute path
        assert!(pathbuf.is_absolute());

        Source {
            path: pathbuf.clone(),
            id: hash_path_string(path.as_ref().to_string_lossy().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATHSTR: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn test_source_new() {
        let s = Source::new(PATHSTR);

        assert_eq!(s.id, hash_path_string(PATHSTR.to_string()));
    }

    #[test]
    fn test_source_json() {
        let source_id = hash_path_string(PATHSTR.to_string());
        let source = Source::new(PATHSTR);
        let source_json_value = serde_json::to_string_pretty(&source).unwrap();

        let from_string = serde_json::from_str::<Source>(&source_json_value).unwrap();
        assert_eq!(from_string.id, source_id);
    }
}
