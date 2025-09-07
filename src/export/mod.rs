//! Export module for saving market data to various formats
//!
//! This module provides functionality to export generated market data to different
//! file formats for analysis and integration with external tools.
//!
//! ## Supported Export Formats
//!
//! - **CSV**: Comma-separated values format
//! - **JSON**: Standard JSON and JSON Lines formats
//! - **CouchDB**: NoSQL database storage
//! - **PNG**: Chart visualization as PNG images
//!
//! ## Usage
//!
//! ```no_run
//! use market_data_source::{MarketDataGenerator, export::{ExportFormat, ExportOptions}};
//!
//! let mut generator = MarketDataGenerator::new();
//! let data = generator.generate_series(100);
//!
//! // Export to CSV
//! market_data_source::export::to_csv_ohlc(&data, "data.csv")?;
//!
//! // Export to JSON
//! market_data_source::export::to_json_ohlc(&data, "data.json")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[cfg(feature = "csv_export")]
pub mod csv;

#[cfg(feature = "csv_export")]
pub use self::csv::{CsvExporter, CsvOptions};

#[cfg(feature = "json_export")]
pub mod json;

#[cfg(feature = "json_export")]
pub use self::json::{JsonExporter, JsonOptions};

#[cfg(feature = "couchdb")]
pub mod couchdb;

#[cfg(feature = "couchdb")]
pub use self::couchdb::{CouchDbExporter, CouchDbOptions};

#[cfg(feature = "png_export")]
pub mod chart;

#[cfg(feature = "png_export")]
pub use self::chart::{ChartBuilder, ChartExporter};

pub mod error;

pub use error::{ExportError, ExportResult};

use crate::types::{OHLC, Tick};
use std::fmt;
use std::path::Path;
use std::io::Write;

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// CSV (Comma-Separated Values) format
    Csv,
    /// JSON format
    Json,
    /// JSON Lines format (one JSON object per line)
    JsonLines,
    /// CouchDB database
    CouchDb,
    /// PNG chart image
    Png,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Csv => write!(f, "CSV"),
            ExportFormat::Json => write!(f, "JSON"),
            ExportFormat::JsonLines => write!(f, "JSON Lines"),
            ExportFormat::CouchDb => write!(f, "CouchDB"),
            ExportFormat::Png => write!(f, "PNG"),
        }
    }
}

/// Options for export operations
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// Include headers in export (applicable to CSV)
    pub include_headers: bool,
    /// Pretty print JSON output
    pub pretty_json: bool,
    /// Custom delimiter for CSV (default: comma)
    pub csv_delimiter: u8,
    /// Include timestamp in output
    pub include_timestamp: bool,
    /// Maximum number of records to export (None for all)
    pub max_records: Option<usize>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            include_headers: true,
            pretty_json: false,
            csv_delimiter: b',',
            include_timestamp: true,
            max_records: None,
        }
    }
}

impl ExportOptions {
    /// Create new export options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to include headers
    pub fn include_headers(mut self, include: bool) -> Self {
        self.include_headers = include;
        self
    }

    /// Set whether to pretty print JSON
    pub fn pretty_json(mut self, pretty: bool) -> Self {
        self.pretty_json = pretty;
        self
    }

    /// Set CSV delimiter
    pub fn csv_delimiter(mut self, delimiter: u8) -> Self {
        self.csv_delimiter = delimiter;
        self
    }

    /// Set whether to include timestamp
    pub fn include_timestamp(mut self, include: bool) -> Self {
        self.include_timestamp = include;
        self
    }

    /// Set maximum number of records to export
    pub fn max_records(mut self, max: Option<usize>) -> Self {
        self.max_records = max;
        self
    }
}

/// Common trait for data exporters
pub trait DataExporter {
    /// Export OHLC data to a file
    fn export_ohlc<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()>;
    
    /// Export tick data to a file
    fn export_ticks<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()>;
    
    /// Export OHLC data to a writer
    fn export_ohlc_to_writer<W: Write>(&self, data: &[OHLC], writer: W) -> ExportResult<()>;
    
    /// Export tick data to a writer
    fn export_ticks_to_writer<W: Write>(&self, data: &[Tick], writer: W) -> ExportResult<()>;
}

/// Convenience function to export OHLC data to CSV
#[cfg(feature = "csv_export")]
pub fn to_csv_ohlc<P: AsRef<Path>>(data: &[OHLC], path: P) -> ExportResult<()> {
    let exporter = CsvExporter::default();
    exporter.export_ohlc(data, path)
}

/// Convenience function to export tick data to CSV
#[cfg(feature = "csv_export")]
pub fn to_csv_ticks<P: AsRef<Path>>(data: &[Tick], path: P) -> ExportResult<()> {
    let exporter = CsvExporter::default();
    exporter.export_ticks(data, path)
}

/// Convenience function to export OHLC data to CSV string
#[cfg(feature = "csv_export")]
pub fn to_csv_string_ohlc(data: &[OHLC]) -> ExportResult<String> {
    use ::csv::WriterBuilder;
    let mut buffer = Vec::new();
    {
        let mut writer = WriterBuilder::new()
            .has_headers(true)
            .from_writer(&mut buffer);
        
        writer.write_record(&["timestamp", "open", "high", "low", "close", "volume"])
            .map_err(|e| ExportError::WriteFailed(e.to_string()))?;
        
        for ohlc in data {
            writer.write_record(&[
                ohlc.timestamp.to_string(),
                ohlc.open.to_string(),
                ohlc.high.to_string(),
                ohlc.low.to_string(),
                ohlc.close.to_string(),
                ohlc.volume.value.to_string(),
            ])
            .map_err(|e| ExportError::WriteFailed(e.to_string()))?;
        }
        
        writer.flush()
            .map_err(|e| ExportError::WriteFailed(e.to_string()))?;
    }
    
    String::from_utf8(buffer)
        .map_err(|e| ExportError::WriteFailed(e.to_string()))
}

/// Convenience function to export OHLC data to JSON
#[cfg(feature = "json_export")]
pub fn to_json_ohlc<P: AsRef<Path>>(data: &[OHLC], path: P) -> ExportResult<()> {
    let exporter = JsonExporter::default();
    exporter.export_ohlc(data, path)
}

/// Convenience function to export tick data to JSON
#[cfg(feature = "json_export")]
pub fn to_json_ticks<P: AsRef<Path>>(data: &[Tick], path: P) -> ExportResult<()> {
    let exporter = JsonExporter::default();
    exporter.export_ticks(data, path)
}

/// Convenience function to export OHLC data to JSON Lines format
#[cfg(feature = "json_export")]
pub fn to_jsonl_ohlc<P: AsRef<Path>>(data: &[OHLC], path: P) -> ExportResult<()> {
    let exporter = JsonExporter::with_options(JsonOptions::json_lines());
    exporter.export_ohlc(data, path)
}

/// Convenience function to export tick data to JSON Lines format
#[cfg(feature = "json_export")]
pub fn to_jsonl_ticks<P: AsRef<Path>>(data: &[Tick], path: P) -> ExportResult<()> {
    let exporter = JsonExporter::with_options(JsonOptions::json_lines());
    exporter.export_ticks(data, path)
}

/// Convenience function to export OHLC data to CouchDB
#[cfg(feature = "couchdb")]
pub fn to_couchdb_ohlc(data: &[OHLC], server_url: &str, database: &str) -> ExportResult<()> {
    let exporter = CouchDbExporter::new(server_url, database);
    exporter.export_ohlc(data, "")
}

/// Convenience function to export tick data to CouchDB
#[cfg(feature = "couchdb")]
pub fn to_couchdb_ticks(data: &[Tick], server_url: &str, database: &str) -> ExportResult<()> {
    let exporter = CouchDbExporter::new(server_url, database);
    exporter.export_ticks(data, "")
}

/// Convenience function to export OHLC data to CouchDB using environment variables
#[cfg(all(feature = "couchdb", feature = "dotenvy"))]
pub fn to_couchdb_ohlc_env(data: &[OHLC]) -> ExportResult<()> {
    CouchDbExporter::from_env().export_ohlc(data, "")
}

/// Convenience function to export tick data to CouchDB using environment variables
#[cfg(all(feature = "couchdb", feature = "dotenvy"))]
pub fn to_couchdb_ticks_env(data: &[Tick]) -> ExportResult<()> {
    CouchDbExporter::from_env().export_ticks(data, "")
}

/// Convenience function to export OHLC data as a candlestick chart PNG
#[cfg(feature = "png_export")]
pub fn to_png_ohlc<P: AsRef<Path>>(data: &[OHLC], path: P) -> ExportResult<()> {
    let exporter = ChartExporter::default();
    exporter.export_ohlc(data, path).map_err(|e| {
        ExportError::Chart(format!("Failed to export OHLC chart: {e}"))
    })
}

/// Convenience function to export tick data as a line chart PNG
#[cfg(feature = "png_export")]
pub fn to_png_ticks<P: AsRef<Path>>(data: &[Tick], path: P) -> ExportResult<()> {
    let exporter = ChartExporter::default();
    exporter.export_ticks(data, path).map_err(|e| {
        ExportError::Chart(format!("Failed to export tick chart: {e}"))
    })
}

/// Convenience function to export OHLC data as a candlestick chart PNG with custom builder
#[cfg(feature = "png_export")]
pub fn to_png_ohlc_with_builder<P: AsRef<Path>>(
    data: &[OHLC],
    path: P,
    builder: ChartBuilder,
) -> ExportResult<()> {
    let exporter = ChartExporter::with_builder(builder);
    exporter.export_ohlc(data, path).map_err(|e| {
        ExportError::Chart(format!("Failed to export OHLC chart: {e}"))
    })
}

/// Convenience function to export tick data as a line chart PNG with custom builder
#[cfg(feature = "png_export")]
pub fn to_png_ticks_with_builder<P: AsRef<Path>>(
    data: &[Tick],
    path: P,
    builder: ChartBuilder,
) -> ExportResult<()> {
    let exporter = ChartExporter::with_builder(builder);
    exporter.export_ticks(data, path).map_err(|e| {
        ExportError::Chart(format!("Failed to export tick chart: {e}"))
    })
}
