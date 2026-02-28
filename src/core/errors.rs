//! Error types

use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PiError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("Extension error: {0}")]
    Extension(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, PiError>;

impl serde::Serialize for PiError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
