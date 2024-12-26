use std::path::PathBuf;
use thiserror::Error;
use url::ParseError;
use validator::ValidationErrors;

/// Represents all possible errors that can occur when using the Firecracker client.
#[derive(Error, Debug)]
pub enum FirecrackerError {
    /// Error occurred during HTTP client operations
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    /// Error parsing URLs
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError),

    /// Error during serialization/deserialization
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Error validating input
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    /// Error from Firecracker API
    #[error("Firecracker API error: {status_code} - {message}")]
    Api { status_code: u16, message: String },

    /// Error with invalid paths
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Error accessing files or paths
    #[error("File system error for path {path}: {source}")]
    FileSystem {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Error during snapshot operations
    #[error("Snapshot error: {0}")]
    Snapshot(String),

    /// Error with rate limiting
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Error with VM state
    #[error("Invalid VM state: {current_state}. Expected one of: {expected_states:?}")]
    InvalidState {
        current_state: String,
        expected_states: Vec<String>,
    },

    /// Timeout error
    #[error("Operation timed out after {duration_secs} seconds")]
    Timeout { duration_secs: u64 },

    /// Generic error for cases that don't fit other categories
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for Firecracker operations
pub type FirecrackerResult<T> = Result<T, FirecrackerError>;
