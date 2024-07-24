use super::package_json_walker::*;
use anyhow::{bail, Result};
use package_json::{PackageJson, PackageJsonManager};
use std::cell::{Ref, RefCell, RefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    pub dir: PathBuf,
    pub package_json_manager: RefCell<PackageJsonManager>,
}

impl Workspace {
    pub fn init_from_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let pkg = PackageJsonManager::with_file_path(path.clone().join("package.json"));

        Workspace {
            dir: path,
            package_json_manager: RefCell::new(pkg),
        }
    }

    /// see https://docs.rs/package-json/0.4.0/package_json/struct.PackageJsonManager.html#method.read_ref
    /// don't know why it must require mut self, so we have to use RefMut here.
    pub fn package_json_manager(&self) -> RefMut<'_, PackageJsonManager> {
        self.package_json_manager.borrow_mut()
    }
    pub fn package_json(&self) -> RefMut<'_, PackageJson> {
        let pkg_manager = self.package_json_manager();

        // Is there a way to map RefMut to Ref?
        // seems not, take ref from mut ref is forbidden, as there should be only one mut ref at a
        // time,
        // or mutiple immutable ref at a time.
        RefMut::map(pkg_manager, |pkg_manager: &mut PackageJsonManager| {
            pkg_manager.read_mut().unwrap()
        })
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
    pub fn walk_package_jsons(&self) -> Result<impl Iterator<Item = PathBuf> + '_> {
        if !self.is_npm_workspaces_project() {
            return Ok(Box::new(::std::iter::empty()) as Box<dyn Iterator<Item = PathBuf>>);
        }

        walk_package_jsons_under_path(&self.dir)
            .map(|x| Box::new(x) as Box<dyn Iterator<Item = PathBuf>>)
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
