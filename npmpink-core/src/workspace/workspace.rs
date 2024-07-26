use super::package_json_walker::*;
use anyhow::Result;
use lazycell::LazyCell;
use package_json_schema::{PackageJson, Workspaces};
use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    pub dir: PathBuf,
    pub package_json: LazyCell<PackageJson>,
}

impl Workspace {
    pub fn init_from_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf().canonicalize().unwrap();

        Workspace {
            dir: path,
            package_json: LazyCell::new(),
        }
    }

    pub fn package_json(&self) -> Result<&PackageJson> {
        let pkg = self.package_json.try_borrow_with(|| {
            let pkg_path = self.absolute_dir()?.clone().join("package.json");
            let pkg_content = fs::read_to_string(pkg_path)?;
            PackageJson::try_from(pkg_content).map_err(anyhow::Error::msg)
        });
        pkg
    }

    /// Check workspace's package.json exists
    pub fn has_package_json(&self) -> bool {
        let pkg_path = self.dir.join("package.json");
        let path_exists_value = pkg_path.try_exists();

        path_exists_value.is_ok() && path_exists_value.unwrap()
    }

    pub fn absolute_dir(&self) -> Result<PathBuf> {
        if self.dir.is_absolute() {
            Ok(self.dir.clone())
        } else {
            self.dir.canonicalize().map_err(anyhow::Error::msg)
        }
    }

    fn is_npm_workspaces_project(&self) -> Result<bool> {
        let pkg = self.package_json()?;
        match pkg.workspaces {
            Some(Workspaces::List(ref list)) => Ok(!list.is_empty()),
            Some(Workspaces::Object { ref packages, .. }) => {
                Ok(packages.is_some() && !packages.clone().unwrap().is_empty())
            }
            _ => Ok(false),
        }
    }

    /// Get package jsons under current workspace if it is
    /// npm multiple projects workspace.
    pub fn walk_package_jsons(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.is_npm_workspaces_project()
            .unwrap()
            .then_some(())
            .and_then(|_| walk_package_jsons_under_path(&self.dir).ok())
            .into_iter()
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;

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
        assert!(dir.is_ok());
    }
}
