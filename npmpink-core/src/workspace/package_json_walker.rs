use anyhow::Result;
use std::path::{Path, PathBuf};
use wax::{any, Glob};

pub(super) fn walk_package_jsons_under_path(
    path: impl AsRef<Path>,
) -> Result<impl 'static + Iterator<Item = PathBuf>> {
    let glob = Glob::new("**/package.json").unwrap().into_owned();
    let walker = glob
        .walk(path)
        .not(any([
            "**/node_modules/**",
            "**/dist/**",
            "**/src/**",
            "**/public/**",
            "**/.git/**",
            "**/.direnv/**",
        ]))
        .unwrap();

    Ok(walker
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path().to_path_buf())
        // clone the path from the iterator results
        .collect::<Vec<PathBuf>>()
        .into_iter())
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
