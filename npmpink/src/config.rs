use crate::source::Source;
#[allow(unused_imports)]
use home::home_dir as crate_home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::{path::PathBuf, result::Result};

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
    crate_home_dir()
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

// maybe move this to config_health.rs module.
/// Health check for config
#[derive(Debug, PartialEq)]
pub(crate) enum HealthCheckError {
    ConfigFileNotExist,
    ConfigFileInvalid,
}

impl std::fmt::Display for HealthCheckError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthCheckError::ConfigFileNotExist => write!(fmt, "Config file not exist"),
            HealthCheckError::ConfigFileInvalid => write!(fmt, "Config file is invalid"),
        }
    }
}

impl Config {
    pub(crate) fn root_config_path() -> &'static PathBuf {
        &ROOT_CONFIG_PATH
    }

    /// Do healthy check for root config
    pub(crate) fn healthcheck() -> Result<(), HealthCheckError> {
        use std::path;

        let config_path = Config::root_config_path();

        // how to make this error handle better
        if !path::Path::new(config_path).try_exists().unwrap() {
            return Err(HealthCheckError::ConfigFileNotExist);
        }

        println!("config path: {:?}", config_path);

        Ok(())
    }

    pub(crate) fn init_from_default() -> Result<(), std::io::Error> {
        let root_config_path = Self::root_config_path();

        let mut file = fs::File::create(root_config_path)?;
        file.write_all(b"testing")?;

        Ok(())
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
    use std::path::{Path, PathBuf};

    #[test]
    fn path_try_exists() {
        assert!(!Path::new("not----existscret_file.txt")
            .try_exists()
            .unwrap(),);
    }

    #[test]
    fn test_workspace_config() {
        let _ = WorkspaceConfig::default();
    }

    #[test]
    fn test_config_health_check_not_exists() {
        let _result = Config::healthcheck();
        assert!(matches!(
            Result::<(), HealthCheckError>::Err(HealthCheckError::ConfigFileNotExist),
            _result
        ));
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
