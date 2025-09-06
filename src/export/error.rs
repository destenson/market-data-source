//! Error types for export operations

use std::fmt;

/// Errors that can occur during export operations
#[derive(Debug)]
pub enum ExportError {
    /// I/O error (file operations, network)
    Io(std::io::Error),
    /// Serialization error (CSV, JSON formatting)
    Serialization(String),
    /// Configuration error (invalid options)
    Configuration(String),
    /// Database error (CouchDB operations)
    Database(String),
    /// Chart rendering error (PNG generation)
    Chart(String),
    /// Invalid data error (malformed input)
    InvalidData(String),
    /// Feature not available (disabled feature flag)
    FeatureNotAvailable(String),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::Io(err) => write!(f, "I/O error: {}", err),
            ExportError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            ExportError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            ExportError::Database(msg) => write!(f, "Database error: {}", msg),
            ExportError::Chart(msg) => write!(f, "Chart rendering error: {}", msg),
            ExportError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ExportError::FeatureNotAvailable(msg) => write!(f, "Feature not available: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExportError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::Io(err)
    }
}

impl From<csv::Error> for ExportError {
    fn from(err: csv::Error) -> Self {
        ExportError::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for ExportError {
    fn from(err: serde_json::Error) -> Self {
        ExportError::Serialization(err.to_string())
    }
}

/// Result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;