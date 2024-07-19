mod package_json_walker;

use anyhow::Result;
use package_json::PackageJsonManager;
use package_json_walker::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Workspace {
    pub(crate) dir: PathBuf,
    pub(crate) package_json_manager: PackageJsonManager,
}

impl Workspace {
    pub(crate) fn init_from_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let pkg = PackageJsonManager::with_file_path(path.clone().join("package.json"));

        Workspace {
            dir: path,
            package_json_manager: pkg,
        }
    }

    /// Check workspace's package.json exists
    pub(crate) fn is_ok_loosely(&self) -> bool {
        let pkg_path = self.dir.join("package.json");
        let path_exists_value = pkg_path.try_exists();

        path_exists_value.is_ok() && path_exists_value.unwrap()
    }

    pub(crate) fn absolute_dir(&self) -> Option<PathBuf> {
        if self.dir.is_absolute() {
            Some(self.dir.clone())
        } else {
            self.dir.canonicalize().ok()
        }
    }

    fn is_npm_workspaces_project(&mut self) -> bool {
        let Some(pkg) = self.package_json_manager.read_ref().ok() else {
            return false;
        };

        if pkg.workspaces.as_ref().map_or(false, |w| !w.is_empty()) {
            return true;
        }

        // TODO: check pnpm workspace lockfile.

        false
    }

    /// Get package jsons under current workspace if it is
    /// npm multiple projects workspace.
    pub(crate) fn get_package_jsons(&mut self) -> Result<impl Iterator<Item = PathBuf> + '_> {
        if !self.is_npm_workspaces_project() {
            // return Ok(std::iter::empty::<PathBuf>());
        }

        walk_package_jsons_under_path(&self.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;

    #[test]
    fn test_workspace_init_from_path_error() {
        let mut wk = Workspace::init_from_dir("/path_that_must_not_exit");
        let pkg = &mut wk.package_json_manager;

        assert!(pkg.read_ref().is_err());
    }

    #[test]
    fn test_workspace_init_from_realpath() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "assets_/dummy/");
        let mut wk = Workspace::init_from_dir(pkg_path);
        let pkg = &mut wk.package_json_manager;

        assert!(pkg.read_ref().is_ok());
    }

    #[test]
    fn test_workspace_ok_loosely() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "assets_/dummy/");
        let wk = Workspace::init_from_dir(pkg_path);

        assert!(wk.is_ok_loosely());
    }

    #[test]
    fn test_workspace_abs_path() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "./assets_/dummy/");
        let wk = Workspace::init_from_dir(pkg_path);

        let dir = wk.absolute_dir();
        assert!(dir.is_some());
    }
}
