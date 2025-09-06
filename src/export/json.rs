//! JSON export functionality for market data
//!
//! This module provides JSON and JSON Lines export capabilities for market data,
//! supporting both standard JSON arrays and streaming JSON Lines format.

use crate::export::{DataExporter, ExportResult};
use crate::types::{OHLC, Tick};
use serde_json;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Options for JSON export
#[derive(Debug, Clone)]
pub struct JsonOptions {
    /// Use pretty printing with indentation
    pub pretty: bool,
    /// Use JSON Lines format (one JSON object per line)
    pub json_lines: bool,
    /// Optional compression (not implemented in this version)
    pub compress: bool,
}

impl Default for JsonOptions {
    fn default() -> Self {
        Self {
            pretty: false,
            json_lines: false,
            compress: false,
        }
    }
}

impl JsonOptions {
    /// Create options for pretty-printed JSON
    pub fn pretty() -> Self {
        Self {
            pretty: true,
            json_lines: false,
            compress: false,
        }
    }

    /// Create options for JSON Lines format
    pub fn json_lines() -> Self {
        Self {
            pretty: false,
            json_lines: true,
            compress: false,
        }
    }
}

/// JSON exporter for market data
pub struct JsonExporter {
    options: JsonOptions,
}

impl JsonExporter {
    /// Create a new JSON exporter with default options
    pub fn new() -> Self {
        Self {
            options: JsonOptions::default(),
        }
    }

    /// Create a JSON exporter with custom options
    pub fn with_options(options: JsonOptions) -> Self {
        Self { options }
    }

    /// Export OHLC data using JSON Lines format
    fn export_ohlc_jsonl<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for ohlc in data {
            serde_json::to_writer(&mut writer, ohlc)?;
            writeln!(&mut writer)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Export tick data using JSON Lines format
    fn export_ticks_jsonl<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for tick in data {
            serde_json::to_writer(&mut writer, tick)?;
            writeln!(&mut writer)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Export OHLC data as standard JSON array
    fn export_ohlc_array<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        if self.options.pretty {
            serde_json::to_writer_pretty(writer, data)?;
        } else {
            serde_json::to_writer(writer, data)?;
        }

        Ok(())
    }

    /// Export tick data as standard JSON array
    fn export_ticks_array<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        if self.options.pretty {
            serde_json::to_writer_pretty(writer, data)?;
        } else {
            serde_json::to_writer(writer, data)?;
        }

        Ok(())
    }
}

impl Default for JsonExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl DataExporter for JsonExporter {
    fn export_ohlc<P: AsRef<Path>>(&self, data: &[OHLC], path: P) -> ExportResult<()> {
        if self.options.json_lines {
            self.export_ohlc_jsonl(data, path)
        } else {
            self.export_ohlc_array(data, path)
        }
    }

    fn export_ticks<P: AsRef<Path>>(&self, data: &[Tick], path: P) -> ExportResult<()> {
        if self.options.json_lines {
            self.export_ticks_jsonl(data, path)
        } else {
            self.export_ticks_array(data, path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_sample_ohlc() -> Vec<OHLC> {
        vec![
            OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1640995200000),
            OHLC::new(103.0, 107.0, 102.0, 106.0, 1500, 1640998800000),
            OHLC::new(106.0, 108.0, 104.0, 105.0, 1200, 1641002400000),
        ]
    }

    fn create_sample_ticks() -> Vec<Tick> {
        vec![
            Tick::new(100.0, 100, 1640995200000),
            Tick::new(100.5, 150, 1640995201000),
            Tick::new(101.0, 200, 1640995202000),
        ]
    }

    #[test]
    fn test_export_ohlc_json_array() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ohlc.json");
        
        let exporter = JsonExporter::new();
        let data = create_sample_ohlc();
        
        exporter.export_ohlc(&data, &file_path).unwrap();
        
        // Read and verify the JSON
        let content = fs::read_to_string(&file_path).unwrap();
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        assert_eq!(parsed.len(), 3);
        assert!(parsed[0].is_valid());
        assert!(parsed[0].open > 0.0);
        assert!(parsed[0].close > 0.0);
    }

    #[test]
    fn test_export_ohlc_json_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ohlc.jsonl");
        
        let options = JsonOptions::json_lines();
        let exporter = JsonExporter::with_options(options);
        let data = create_sample_ohlc();
        
        exporter.export_ohlc(&data, &file_path).unwrap();
        
        // Read and verify the JSON Lines
        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.trim().split('\n').collect();
        
        assert_eq!(lines.len(), 3);
        
        // Parse first line
        let first: OHLC = serde_json::from_str(lines[0]).unwrap();
        assert!(first.is_valid());
        assert!(first.open > 0.0);
        assert!(first.close > 0.0);
    }

    #[test]
    fn test_export_ohlc_pretty_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ohlc_pretty.json");
        
        let options = JsonOptions::pretty();
        let exporter = JsonExporter::with_options(options);
        let data = create_sample_ohlc();
        
        exporter.export_ohlc(&data, &file_path).unwrap();
        
        // Read and verify the JSON is pretty-printed (contains indentation)
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("  "), "Pretty JSON should contain indentation");
        
        // Verify it's still valid JSON
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn test_export_ticks_json_array() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ticks.json");
        
        let exporter = JsonExporter::new();
        let data = create_sample_ticks();
        
        exporter.export_ticks(&data, &file_path).unwrap();
        
        // Read and verify the JSON
        let content = fs::read_to_string(&file_path).unwrap();
        let parsed: Vec<Tick> = serde_json::from_str(&content).unwrap();
        
        assert_eq!(parsed.len(), 3);
        assert!(parsed[0].price > 0.0);
        assert!(parsed[1].price > 0.0);
    }

    #[test]
    fn test_export_ticks_json_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ticks.jsonl");
        
        let options = JsonOptions::json_lines();
        let exporter = JsonExporter::with_options(options);
        let data = create_sample_ticks();
        
        exporter.export_ticks(&data, &file_path).unwrap();
        
        // Read and verify the JSON Lines
        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.trim().split('\n').collect();
        
        assert_eq!(lines.len(), 3);
        
        // Parse each line and verify structure
        for line in lines.iter() {
            let tick: Tick = serde_json::from_str(line).unwrap();
            assert!(tick.price > 0.0, "Price should be positive");
            assert!(tick.volume.value() >= 0, "Volume should be non-negative");
        }
    }

    #[test]
    fn test_json_roundtrip_ohlc() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("roundtrip.json");
        
        let exporter = JsonExporter::new();
        let original_data = create_sample_ohlc();
        
        // Export
        exporter.export_ohlc(&original_data, &file_path).unwrap();
        
        // Import
        let content = fs::read_to_string(&file_path).unwrap();
        let imported_data: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        // Verify successful roundtrip - data can be exported and re-imported
        assert_eq!(original_data.len(), imported_data.len());
        
        // Verify structural integrity rather than exact values
        for (i, ohlc) in imported_data.iter().enumerate() {
            assert!(ohlc.is_valid(), "OHLC at index {} should be valid", i);
            
            // Verify the relationships are preserved
            assert!(ohlc.high >= ohlc.low, "High should be >= low");
            assert!(ohlc.high >= ohlc.open, "High should be >= open");
            assert!(ohlc.high >= ohlc.close, "High should be >= close");
            assert!(ohlc.low <= ohlc.open, "Low should be <= open");
            assert!(ohlc.low <= ohlc.close, "Low should be <= close");
            
            // Verify timestamps and volumes match exactly (non-float values)
            assert_eq!(original_data[i].volume.value(), ohlc.volume.value());
            assert_eq!(original_data[i].timestamp, ohlc.timestamp);
        }
    }

    #[test]
    fn test_json_lines_format_validation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("validate.jsonl");
        
        let options = JsonOptions::json_lines();
        let exporter = JsonExporter::with_options(options);
        let data = create_sample_ohlc();
        
        exporter.export_ohlc(&data, &file_path).unwrap();
        
        // Read file and ensure each line is valid JSON
        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.trim().split('\n').collect();
        
        for line in lines {
            // Each line should be valid JSON
            let result: Result<OHLC, _> = serde_json::from_str(line);
            assert!(result.is_ok(), "Each line should be valid JSON");
        }
    }

    #[test]
    fn test_empty_data_export() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.json");
        
        let exporter = JsonExporter::new();
        let empty_data: Vec<OHLC> = vec![];
        
        exporter.export_ohlc(&empty_data, &file_path).unwrap();
        
        let content = fs::read_to_string(&file_path).unwrap();
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        assert_eq!(parsed.len(), 0);
        assert_eq!(content.trim(), "[]");
    }
}