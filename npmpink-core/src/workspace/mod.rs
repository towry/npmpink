mod package_json_walker;

use anyhow::{anyhow, bail, Result};
use lazycell::LazyCell;
use package_json::PackageJsonManager;
use package_json_walker::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use crate::lockfile::LockfileContent;

#[derive(Debug)]
pub struct Workspace {
    pub dir: PathBuf,
    pub package_json_manager: RefCell<PackageJsonManager>,
    pub lockfile: LazyCell<LockfileContent>,
}

impl Workspace {
    pub fn init_from_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let pkg = PackageJsonManager::with_file_path(path.clone().join("package.json"));

        Workspace {
            dir: path,
            package_json_manager: RefCell::new(pkg),
            lockfile: LazyCell::new(),
        }
    }

    /// Check workspace's package.json exists
    pub fn has_package_json(&self) -> bool {
        let pkg_path = self.dir.join("package.json");
        let path_exists_value = pkg_path.try_exists();

        path_exists_value.is_ok() && path_exists_value.unwrap()
    }

    pub fn absolute_dir(&self) -> Option<PathBuf> {
        if self.dir.is_absolute() {
            Some(self.dir.clone())
        } else {
            self.dir.canonicalize().ok()
        }
    }

    pub fn lockfile_path(&self) -> Option<PathBuf> {
        let mut dir = self.absolute_dir()?;
        dir.push("npmpink.lock");
        Some(dir)
    }

    fn is_npm_workspaces_project(&self) -> bool {
        let mut pkg = self.package_json_manager.borrow_mut();
        let Some(pkg) = pkg.read_ref().ok() else {
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
    pub fn package_jsons(&self) -> Result<impl Iterator<Item = PathBuf> + '_> {
        if !self.is_npm_workspaces_project() {
            bail!("not workspaces");
        }

        walk_package_jsons_under_path(&self.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;

    #[test]
    fn test_workspace_init_from_path_error() {
        let wk = Workspace::init_from_dir("/path_that_must_not_exit");
        let pkg = &mut wk.package_json_manager.borrow_mut();

        assert!(pkg.read_ref().is_err());
    }

    #[test]
    fn test_workspace_init_from_realpath() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "assets_/dummy/");
        let wk = Workspace::init_from_dir(pkg_path);
        let pkg = &mut wk.package_json_manager.borrow_mut();

        assert!(pkg.read_ref().is_ok());
    }

    #[test]
    fn test_workspace_ok_loosely() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "assets_/dummy/");
        let wk = Workspace::init_from_dir(pkg_path);

        assert!(wk.has_package_json());
    }

    #[test]
    fn test_workspace_abs_path() {
        let pkg_path = concat!(env!("CARGO_WORKSPACE_DIR"), "./assets_/dummy/");
        let wk = Workspace::init_from_dir(pkg_path);

        let dir = wk.absolute_dir();
        assert!(dir.is_some());
    }
}
