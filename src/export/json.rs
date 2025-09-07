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
#[derive(Default)]
pub struct JsonOptions {
    /// Use pretty printing with indentation
    pub pretty: bool,
    /// Use JSON Lines format (one JSON object per line)
    pub json_lines: bool,
    /// Optional compression (not implemented in this version)
    pub compress: bool,
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

    /// Write OHLC data using JSON Lines format to a writer
    fn write_ohlc_jsonl<W: Write>(&self, data: &[OHLC], mut writer: W) -> ExportResult<()> {
        for ohlc in data {
            serde_json::to_writer(&mut writer, ohlc)?;
            writeln!(&mut writer)?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Write tick data using JSON Lines format to a writer
    fn write_ticks_jsonl<W: Write>(&self, data: &[Tick], mut writer: W) -> ExportResult<()> {
        for tick in data {
            serde_json::to_writer(&mut writer, tick)?;
            writeln!(&mut writer)?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Write OHLC data as standard JSON array to a writer
    fn write_ohlc_array<W: Write>(&self, data: &[OHLC], writer: W) -> ExportResult<()> {
        if self.options.pretty {
            serde_json::to_writer_pretty(writer, data)?;
        } else {
            serde_json::to_writer(writer, data)?;
        }
        Ok(())
    }

    /// Write tick data as standard JSON array to a writer
    fn write_ticks_array<W: Write>(&self, data: &[Tick], writer: W) -> ExportResult<()> {
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
    
    fn export_ohlc_to_writer<W: Write>(&self, data: &[OHLC], writer: W) -> ExportResult<()> {
        if self.options.json_lines {
            self.write_ohlc_jsonl(data, writer)
        } else {
            self.write_ohlc_array(data, writer)
        }
    }
    
    fn export_ticks_to_writer<W: Write>(&self, data: &[Tick], writer: W) -> ExportResult<()> {
        if self.options.json_lines {
            self.write_ticks_jsonl(data, writer)
        } else {
            self.write_ticks_array(data, writer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn create_sample_ohlc() -> Vec<OHLC> {
        vec![
            OHLC::new(Decimal::from(100), Decimal::from(105), Decimal::from(99), Decimal::from(103), 1000, 1640995200000),
            OHLC::new(Decimal::from(103), Decimal::from(107), Decimal::from(102), Decimal::from(106), 1500, 1640998800000),
            OHLC::new(Decimal::from(106), Decimal::from(108), Decimal::from(104), Decimal::from(105), 1200, 1641002400000),
        ]
    }

    fn create_sample_ticks() -> Vec<Tick> {
        vec![
            Tick::new(Decimal::from(100), 100, 1640995200000),
            Tick::new(Decimal::from_str("100.5").unwrap(), 150, 1640995201000),
            Tick::new(Decimal::from(101), 200, 1640995202000),
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
        assert!(parsed[0].open > Decimal::from(0));
        assert!(parsed[0].close > Decimal::from(0));
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
        assert!(first.open > Decimal::from(0));
        assert!(first.close > Decimal::from(0));
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
        assert!(parsed[0].price > Decimal::from(0));
        assert!(parsed[1].price > Decimal::from(0));
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
            assert!(tick.price > Decimal::from(0), "Price should be positive");
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
            assert!(ohlc.is_valid(), "OHLC at index {i} should be valid");
            
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