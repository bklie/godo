use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GodoError {
    #[error("godo is not initialized. Run 'godo init' first.")]
    NotInitialized,

    #[error("godo is already initialized at {0}")]
    AlreadyInitialized(PathBuf),

    #[error("Task #{0} not found.")]
    TaskNotFound(u32),

    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directory: {path}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse config file: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    ConfigSerialize(#[from] toml::ser::Error),

    #[error("Could not determine home directory")]
    HomeNotFound,
}

pub type Result<T> = std::result::Result<T, GodoError>;
