use crate::source::Source;
use anyhow::Result;
#[allow(unused_imports)]
use home::home_dir as crate_home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

lazy_static! {
    static ref ROOT_CONFIG_PATH: PathBuf = get_root_config_path();
    pub(crate) static ref appConfig: Mutex<Config> = Mutex::new(Config::load_or_default());
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
#[serde(rename_all = "snake_case")]
pub(crate) enum Mode {
    Symlink,
    Copy,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) mode: Mode,
    pub(crate) sources: Vec<Source>,
}

// maybe move this to config_health.rs module.
/// Health check for config
#[derive(Debug, PartialEq, Error)]
pub(crate) enum HealthCheckError {
    #[error("File does not exist")]
    ConfigFileNotExist,
    #[error("Config file is invalid")]
    ConfigFileInvalid,
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

    pub(crate) fn create_from_default() -> Result<()> {
        let root_config_path = Self::root_config_path();
        let mut file = fs::File::create(root_config_path)?;
        let content = serde_json::to_string_pretty(&Config::default()).unwrap();

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub(crate) fn load_or_default() -> Self {
        let root_config_path = Self::root_config_path();

        if root_config_path.try_exists().unwrap_or(false) {
            let content = fs::read_to_string(root_config_path);
            if let Ok(content) = content {
                return serde_json::from_str(&content).unwrap_or(Config::default());
            }
        }

        Config::default()
    }

    pub(crate) fn has_source(&self, id: &str) -> bool {
        self.sources.iter().any(|s| s.id == id)
    }

    pub(crate) fn flush(&self) -> std::io::Result<()> {
        let root_config_path = Self::root_config_path();
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(root_config_path, content.as_bytes())
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
