//! Error types for export operations

use std::fmt;

/// Errors that can occur during export operations
#[derive(Debug)]
pub enum ExportError {
    /// I/O error (file operations, network)
    Io(std::io::Error),
    /// CSV serialization error
    #[cfg(feature = "csv_export")]
    Csv(csv::Error),
    /// JSON serialization error
    #[cfg(feature = "json_export")]
    Json(serde_json::Error),
    /// CouchDB database error
    #[cfg(feature = "couchdb")]
    CouchDb(couch_rs::error::CouchError),
    #[deprecated(note = "Use specific wrapped error variant instead")]
    /// Configuration error (invalid options)
    Configuration(String),
    #[deprecated(note = "Use specific wrapped error variant instead")]
    /// Chart rendering error (PNG generation)
    Chart(String),
    #[deprecated(note = "Use specific wrapped error variant instead")]
    /// Invalid data error (malformed input)
    InvalidData(String),
    #[deprecated(note = "Use specific wrapped error variant instead")]
    /// Write failed error
    WriteFailed(String),
    /// Feature not available (disabled feature flag)
    FeatureNotAvailable(String),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::Io(err) => write!(f, "I/O error: {}", err),
            #[cfg(feature = "csv_export")]
            ExportError::Csv(err) => write!(f, "CSV error: {}", err),
            #[cfg(feature = "json_export")]
            ExportError::Json(err) => write!(f, "JSON error: {}", err),
            #[cfg(feature = "couchdb")]
            ExportError::CouchDb(err) => write!(f, "CouchDB error: {}", err),
            ExportError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            ExportError::Chart(msg) => write!(f, "Chart rendering error: {}", msg),
            ExportError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ExportError::WriteFailed(msg) => write!(f, "Write failed: {}", msg),
            ExportError::FeatureNotAvailable(msg) => write!(f, "Feature not available: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExportError::Io(err) => Some(err),
            #[cfg(feature = "csv_export")]
            ExportError::Csv(err) => Some(err),
            #[cfg(feature = "json_export")]
            ExportError::Json(err) => Some(err),
            #[cfg(feature = "couchdb")]
            ExportError::CouchDb(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::Io(err)
    }
}

#[cfg(feature = "csv_export")]
impl From<csv::Error> for ExportError {
    fn from(err: csv::Error) -> Self {
        ExportError::Csv(err)
    }
}

#[cfg(feature = "json_export")]
impl From<serde_json::Error> for ExportError {
    fn from(err: serde_json::Error) -> Self {
        ExportError::Json(err)
    }
}

#[cfg(feature = "couchdb")]
impl From<couch_rs::error::CouchError> for ExportError {
    fn from(err: couch_rs::error::CouchError) -> Self {
        ExportError::CouchDb(err)
    }
}

/// Result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;
