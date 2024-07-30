use crate::walker;
use anyhow::Result;
use regex::bytes::Regex;
use std::path::{Path, PathBuf};

pub(super) fn walk_package_jsons_under_path(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let wo = walker::WalkOption::new(vec![Regex::new(r"package\.json$").unwrap()]);
    let paths = walker::walk(&[path], Some(wo))?;

    Ok(paths)
}

#[test]
fn test_walk_package_jsons_under_path() {
    let workspace_root = concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "assets_/fixtures_npm_workspaces"
    );

    let paths = walk_package_jsons_under_path(workspace_root).unwrap();
    assert!(paths.len() == 5);
}
