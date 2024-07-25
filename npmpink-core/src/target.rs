use crate::lockfile::LockfileContent;
use crate::package::Package;
use crate::workspace::Workspace;
use anyhow::{Context, Result};
use lazycell::LazyCell;
use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Target {
    pub workspace: Workspace,
    pub lockfile: LazyCell<RefCell<LockfileContent>>,
}

impl<'s> Target {
    pub fn init_from_dir(path: impl AsRef<Path>) -> Self {
        Target {
            workspace: Workspace::init_from_dir(path),
            lockfile: LazyCell::new(),
        }
    }

    /// let mut lockfile = target.lockfile().borrow_mut()?;
    pub fn lockfile_mut(&self) -> Result<RefMut<'_, LockfileContent>> {
        let lockfile = self.lockfile.try_borrow_with(|| {
            Ok::<_, anyhow::Error>(RefCell::new(self.load_lockfile_or_default()?))
        })?;
        Ok(lockfile.borrow_mut())
    }

    pub fn lockfile(&'s self) -> Result<Ref<'s, LockfileContent>> {
        let lockfile = self.lockfile.try_borrow_with(|| {
            Ok::<_, anyhow::Error>(RefCell::new(self.load_lockfile_or_default()?))
        })?;
        Ok(lockfile.borrow())
    }

    pub fn flush_lockfile(&self) -> Result<()> {
        let lockfile_path = self.lockfile_path().context("failed to flush lockfile")?;
        let lockfile = self.lockfile().context("failed to get lockfile")?;
        let content = lockfile.to_json_string()?;

        fs::write(lockfile_path, content.as_bytes()).map_err(anyhow::Error::msg)
    }

    fn load_lockfile_or_default(&self) -> Result<LockfileContent> {
        let Some(lockpath) = self.lockfile_path() else {
            return Ok(LockfileContent::default());
        };
        let Some(lock_content) = fs::read_to_string(lockpath).ok() else {
            return Ok(LockfileContent::default());
        };
        LockfileContent::init_from_lockfile_string(lock_content)
    }

    pub fn lockfile_path(&self) -> Option<PathBuf> {
        let mut dir = self.workspace.absolute_dir()?;
        dir.push("npmpink.lock");
        Some(dir)
    }
}

impl Target {
    pub fn packages_iter(&self) -> impl Iterator<Item = Package> + 'static {
        let lockfile = self.lockfile().unwrap();
        lockfile.packages_iter()
    }

    pub fn packages(&self) -> Vec<Package> {
        let lockfile = self.lockfile().unwrap();
        lockfile.packages.values().cloned().collect()
    }
}
