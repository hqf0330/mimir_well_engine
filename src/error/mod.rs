//! Error handling module

use thiserror::Error;

/// Main error type for the application
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("MDL error: {0}")]
    Mdl(String),

    #[error("SQL planning error: {0}")]
    Planning(String),

    #[error("Connector error: {0}")]
    Connector(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
