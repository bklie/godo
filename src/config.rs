use crate::error::{GodoError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub data_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub date_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                data_file: "~/.godo/tasks.md".to_string(),
            },
            display: DisplayConfig {
                date_format: "%Y-%m-%d %H:%M".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| GodoError::FileRead {
            path: path.clone(),
            source: e,
        })?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content).map_err(|e| GodoError::FileWrite {
            path: path.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn data_file_path(&self) -> Result<PathBuf> {
        expand_tilde(&self.general.data_file)
    }
}

pub fn godo_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or(GodoError::HomeNotFound)?;
    Ok(home.join(".godo"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(godo_dir()?.join("config.toml"))
}

pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") {
        let home = dirs::home_dir().ok_or(GodoError::HomeNotFound)?;
        Ok(home.join(&path[2..]))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn is_initialized() -> bool {
    config_path().map(|p| p.exists()).unwrap_or(false)
}
