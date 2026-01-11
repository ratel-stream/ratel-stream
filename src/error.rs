use config::ConfigError;
use std::time::SystemTimeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RatelStreamError {
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Error while running management server: {0}")]
    MgmtServerError(std::io::Error),

    #[error("IO error, error: {0} details: {1}")]
    IoError(String, std::io::Error),

    #[error("Error: {0}")]
    Error(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Directory does not exist: {0}")]
    DirectoryDoesNotExist(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unknown error")]
    Unknown,
}
