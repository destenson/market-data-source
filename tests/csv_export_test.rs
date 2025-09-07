//! Integration tests for CSV export functionality

#![cfg(feature = "csv_export")]

use market_data_source::{MarketDataGenerator, ConfigBuilder};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_export_ohlc_to_csv() {
    // Create a temporary directory for test files
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_ohlc.csv");
    
    // Create generator and generate data
    let config = ConfigBuilder::new()
        .starting_price_f64(100.0)
        .volatility(0.02)
        .seed(42)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    let data = generator.generate_series(10);
    
    // Export to CSV
    market_data_source::export::to_csv_ohlc(&data, &file_path).unwrap();
    
    // Verify file exists
    assert!(file_path.exists());
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&file_path).unwrap();
    
    // Check headers
    assert!(csv_content.contains("timestamp,open,high,low,close,volume"));
    
    // Check we have correct number of rows (1 header + 10 data rows)
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 11);
    
    // Verify first data row has expected format
    let first_data_row = lines[1];
    let fields: Vec<&str> = first_data_row.split(',').collect();
    assert_eq!(fields.len(), 6); // timestamp, open, high, low, close, volume
}

#[test]
fn test_export_ticks_to_csv() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_ticks.csv");
    
    let mut generator = MarketDataGenerator::new();
    let data = generator.generate_ticks(5);
    
    // Export to CSV
    market_data_source::export::to_csv_ticks(&data, &file_path).unwrap();
    
    // Verify file exists
    assert!(file_path.exists());
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&file_path).unwrap();
    
    // Check headers
    assert!(csv_content.contains("timestamp,price,volume,bid,ask"));
    
    // Check we have correct number of rows
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 6); // 1 header + 5 data rows
}

#[test]
fn test_generator_direct_csv_export() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("direct_export.csv");
    
    let config = ConfigBuilder::new()
        .starting_price_f64(50.0)
        .seed(123)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Use convenience method to generate and export
    generator.generate_to_csv_ohlc(20, &file_path).unwrap();
    
    // Verify file exists and has correct content
    assert!(file_path.exists());
    
    let csv_content = fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 21); // 1 header + 20 data rows
}

#[test]
fn test_stream_csv_export() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("stream_export.csv");
    
    let mut generator = MarketDataGenerator::new();
    
    // Use streaming method for large dataset
    let count = generator.stream_generate_to_csv_ohlc(100, &file_path).unwrap();
    
    assert_eq!(count, 100);
    assert!(file_path.exists());
    
    // Verify file has correct number of lines
    let csv_content = fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 101); // 1 header + 100 data rows
}

#[test]
fn test_csv_custom_delimiter() {
    use market_data_source::export::csv::CsvExporter;
    use market_data_source::export::DataExporter;
    
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("semicolon.csv");
    
    let mut generator = MarketDataGenerator::new();
    let data = generator.generate_series(3);
    
    // Create exporter with semicolon delimiter
    let exporter = CsvExporter::new()
        .delimiter(b';');
    
    exporter.export_ohlc(&data, &file_path).unwrap();
    
    // Verify semicolon delimiter is used
    let csv_content = fs::read_to_string(&file_path).unwrap();
    assert!(csv_content.contains("timestamp;open;high;low;close;volume"));
}

#[test]
fn test_csv_no_headers() {
    use market_data_source::export::csv::CsvExporter;
    use market_data_source::export::DataExporter;
    
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("no_headers.csv");
    
    let mut generator = MarketDataGenerator::new();
    let data = generator.generate_ticks(2);
    
    // Create exporter without headers
    let exporter = CsvExporter::new()
        .include_headers(false);
    
    exporter.export_ticks(&data, &file_path).unwrap();
    
    // Verify no headers
    let csv_content = fs::read_to_string(&file_path).unwrap();
    assert!(!csv_content.contains("timestamp,price,volume"));
    
    // Should only have 2 data lines
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_large_dataset_streaming() {
    let temp_dir = tempdir().unwrap();
    let ohlc_path = temp_dir.path().join("large_ohlc.csv");
    let tick_path = temp_dir.path().join("large_ticks.csv");
    
    let mut generator = MarketDataGenerator::new();
    
    // Generate large datasets using streaming
    let ohlc_count = generator.stream_generate_to_csv_ohlc(1000, &ohlc_path).unwrap();
    let tick_count = generator.stream_generate_to_csv_ticks(1000, &tick_path).unwrap();
    
    assert_eq!(ohlc_count, 1000);
    assert_eq!(tick_count, 1000);
    
    // Files should exist and be non-empty
    assert!(ohlc_path.exists());
    assert!(tick_path.exists());
    
    let ohlc_size = fs::metadata(&ohlc_path).unwrap().len();
    let tick_size = fs::metadata(&tick_path).unwrap().len();
    
    assert!(ohlc_size > 0);
    assert!(tick_size > 0);
}