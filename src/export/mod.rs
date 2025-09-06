//! Export module for saving market data to various formats
//!
//! This module provides functionality to export generated market data to different
//! file formats for analysis and integration with external tools.

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

use crate::types::{OHLC, Tick};
use std::error::Error;
use std::path::Path;

/// Result type for export operations
pub type ExportResult<T> = Result<T, Box<dyn Error>>;

/// Common trait for data exporters
pub trait DataExporter {
    /// Export OHLC data to a file
    fn export_ohlc<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()>;
    
    /// Export tick data to a file
    fn export_ticks<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()>;
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
    exporter.export_ohlc(data, path)
}

/// Convenience function to export tick data as a line chart PNG
#[cfg(feature = "png_export")]
pub fn to_png_ticks<P: AsRef<Path>>(data: &[Tick], path: P) -> ExportResult<()> {
    let exporter = ChartExporter::default();
    exporter.export_ticks(data, path)
}

/// Convenience function to export OHLC data as a candlestick chart PNG with custom builder
#[cfg(feature = "png_export")]
pub fn to_png_ohlc_with_builder<P: AsRef<Path>>(
    data: &[OHLC],
    path: P,
    builder: ChartBuilder,
) -> ExportResult<()> {
    let exporter = ChartExporter::with_builder(builder);
    exporter.export_ohlc(data, path)
}

/// Convenience function to export tick data as a line chart PNG with custom builder
#[cfg(feature = "png_export")]
pub fn to_png_ticks_with_builder<P: AsRef<Path>>(
    data: &[Tick],
    path: P,
    builder: ChartBuilder,
) -> ExportResult<()> {
    let exporter = ChartExporter::with_builder(builder);
    exporter.export_ticks(data, path)
}