use crate::source::Source;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

lazy_static! {
    static ref ROOT_CONFIG_PATH: PathBuf = get_root_config_path();
    pub(crate) static ref appConfig: Config = Config::default();
}

fn get_root_config_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| panic!("Can not get the home dir"))
        .join(".npmpink.json")
}

#[cfg(not(test))]
fn home_dir() -> Option<PathBuf> {
    std::env::home_dir()
}
#[cfg(test)]
fn home_dir() -> Option<PathBuf> {
    Some(PathBuf::from(env!("CARGO_WORKSPACE_DIR")))
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Mode {
    Symlink,
    Copy,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    mode: Mode,
    sources: Vec<Source>,
}

impl Config {
    pub(crate) fn root_config_path() -> &'static PathBuf {
        &ROOT_CONFIG_PATH
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: Mode::Symlink,
            sources: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct WorkspaceConfig {
    mode: Mode,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        WorkspaceConfig {
            mode: Mode::Symlink,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_workspace_config() {
        let _ = WorkspaceConfig::default();
    }

    #[test]
    fn test_workspace_root_home_env() {
        let test_home = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        assert!(test_home.is_absolute());
    }

    #[test]
    fn test_get_root_config_path() {
        let config_root_path = Config::root_config_path();
        assert_eq!(
            config_root_path,
            &PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(".npmpink.json")
        );
    }
}
