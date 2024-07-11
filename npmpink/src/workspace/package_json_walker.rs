use anyhow::Result;
use std::path::{Path, PathBuf};
use wax::Glob;

pub(super) fn walk_package_jsons_under_path(
    path: impl AsRef<Path>,
) -> Result<impl Iterator<Item = PathBuf>> {
    let glob = Glob::new("**/package.json")?;
    let walker = glob.walk(path).into_owned();

    Ok(walker
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path().to_path_buf()))
}

#[ignore]
#[test]
fn test_walk_package_jsons_under_path() {
    let workspace_root = concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "assets_/fixtures_npm_workspaces"
    );

    let paths = walk_package_jsons_under_path(workspace_root)
        .unwrap()
        .collect::<Vec<PathBuf>>();
    assert!(paths.len() == 5);
}
