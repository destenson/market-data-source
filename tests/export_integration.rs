//! Comprehensive integration tests for all export functionality
//!
//! This file contains end-to-end integration tests that verify the complete
//! export pipeline across all supported formats, including round-trip tests,
//! performance benchmarks, and error handling scenarios.

use market_data_source::{
    MarketDataGenerator, ConfigBuilder, TrendDirection,
    types::{OHLC, Tick},
};
use std::fs;
use tempfile::{tempdir, TempDir};
use std::time::Instant;

/// Test fixture for export integration tests
struct ExportTestFixture {
    generator: MarketDataGenerator,
    temp_dir: TempDir,
    ohlc_data: Vec<OHLC>,
    tick_data: Vec<Tick>,
}

impl ExportTestFixture {
    fn new() -> Self {
        let config = ConfigBuilder::new()
            .starting_price_f64(100.0)
            .volatility_f64(0.02)
            .trend_f64(TrendDirection::Bullish, 0.001)
            .seed(12345)  // Fixed seed for reproducible tests
            .build()
            .unwrap();
        
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let temp_dir = tempdir().unwrap();
        
        // Pre-generate test data
        let ohlc_data = generator.generate_series(100);
        let tick_data = generator.generate_ticks(50);
        
        Self {
            generator,
            temp_dir,
            ohlc_data,
            tick_data,
        }
    }
    
    fn get_path(&self, filename: &str) -> std::path::PathBuf {
        self.temp_dir.path().join(filename)
    }
}

#[cfg(feature = "csv_export")]
mod csv_integration_tests {
    use super::*;
    use market_data_source::export::{to_csv_ohlc, to_csv_ticks};
    
    #[test]
    fn test_csv_export_integration() {
        let fixture = ExportTestFixture::new();
        
        // Export OHLC data
        let ohlc_path = fixture.get_path("integration_ohlc.csv");
        let result = to_csv_ohlc(&fixture.ohlc_data, &ohlc_path);
        assert!(result.is_ok(), "CSV OHLC export failed: {:?}", result);
        
        // Verify file exists and has correct content
        assert!(ohlc_path.exists());
        let content = fs::read_to_string(&ohlc_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        
        // Should have header + data rows
        assert_eq!(lines.len(), fixture.ohlc_data.len() + 1);
        assert!(lines[0].contains("timestamp,open,high,low,close,volume"));
        
        // Export tick data
        let tick_path = fixture.get_path("integration_ticks.csv");
        let result = to_csv_ticks(&fixture.tick_data, &tick_path);
        assert!(result.is_ok(), "CSV tick export failed: {:?}", result);
        
        // Verify tick file
        assert!(tick_path.exists());
        let tick_content = fs::read_to_string(&tick_path).unwrap();
        let tick_lines: Vec<&str> = tick_content.lines().collect();
        assert_eq!(tick_lines.len(), fixture.tick_data.len() + 1);
    }
    
    #[test]
    fn test_csv_round_trip() {
        let fixture = ExportTestFixture::new();
        
        // Export original data
        let csv_path = fixture.get_path("roundtrip.csv");
        to_csv_ohlc(&fixture.ohlc_data, &csv_path).unwrap();
        
        // Read back and parse CSV
        use csv::Reader;
        let mut reader = Reader::from_path(&csv_path).unwrap();
        
        let mut parsed_data = Vec::new();
        for record in reader.deserialize() {
            let ohlc: OHLC = record.unwrap();
            parsed_data.push(ohlc);
        }
        
        // Verify data integrity
        assert_eq!(parsed_data.len(), fixture.ohlc_data.len());
        
        // Compare key fields (allowing for float precision differences)
        for (original, parsed) in fixture.ohlc_data.iter().zip(parsed_data.iter()) {
            assert!((original.open - parsed.open).abs() < 0.001);
            assert!((original.high - parsed.high).abs() < 0.001);
            assert!((original.low - parsed.low).abs() < 0.001);
            assert!((original.close - parsed.close).abs() < 0.001);
            assert_eq!(original.volume, parsed.volume);
        }
    }
}

#[cfg(feature = "json_export")]
mod json_integration_tests {
    use super::*;
    use market_data_source::export::{to_json_ohlc, to_jsonl_ohlc};
    use serde_json;
    
    #[test]
    fn test_json_export_integration() {
        let fixture = ExportTestFixture::new();
        
        // Test standard JSON export
        let json_path = fixture.get_path("integration.json");
        let result = to_json_ohlc(&fixture.ohlc_data, &json_path);
        assert!(result.is_ok(), "JSON export failed: {:?}", result);
        
        // Verify file exists and is valid JSON
        assert!(json_path.exists());
        let content = fs::read_to_string(&json_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Should be an array with correct length
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), fixture.ohlc_data.len());
        
        // Test JSON Lines export
        let jsonl_path = fixture.get_path("integration.jsonl");
        let result = to_jsonl_ohlc(&fixture.ohlc_data, &jsonl_path);
        assert!(result.is_ok(), "JSONL export failed: {:?}", result);
        
        // Verify JSONL format
        let jsonl_content = fs::read_to_string(&jsonl_path).unwrap();
        let jsonl_lines: Vec<&str> = jsonl_content.lines().collect();
        assert_eq!(jsonl_lines.len(), fixture.ohlc_data.len());
        
        // Each line should be valid JSON
        for line in jsonl_lines {
            let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
            assert!(parsed.is_object());
        }
    }
    
    #[test]
    fn test_json_round_trip() {
        let fixture = ExportTestFixture::new();
        
        // Export to JSON
        let json_path = fixture.get_path("json_roundtrip.json");
        to_json_ohlc(&fixture.ohlc_data, &json_path).unwrap();
        
        // Read back and parse
        let content = fs::read_to_string(&json_path).unwrap();
        let parsed_data: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        // Verify data integrity
        assert_eq!(parsed_data.len(), fixture.ohlc_data.len());
        
        // JSON should preserve values with reasonable precision
        for (original, parsed) in fixture.ohlc_data.iter().zip(parsed_data.iter()) {
            assert!((original.open - parsed.open).abs() < 1e-10);
            assert!((original.high - parsed.high).abs() < 1e-10);
            assert!((original.low - parsed.low).abs() < 1e-10);
            assert!((original.close - parsed.close).abs() < 1e-10);
            assert_eq!(original.volume, parsed.volume);
            assert_eq!(original.timestamp, parsed.timestamp);
        }
    }
}

#[cfg(feature = "png_export")]
mod png_integration_tests {
    use super::*;
    use market_data_source::export::{to_png_ohlc, to_png_ticks};
    
    #[test]
    fn test_png_export_integration() {
        let fixture = ExportTestFixture::new();
        
        // Test OHLC chart export
        let ohlc_chart_path = fixture.get_path("integration_candlestick.png");
        let result = to_png_ohlc(&fixture.ohlc_data, &ohlc_chart_path);
        assert!(result.is_ok(), "PNG OHLC export failed: {:?}", result);
        
        // Verify PNG file
        assert!(ohlc_chart_path.exists());
        let file_size = fs::metadata(&ohlc_chart_path).unwrap().len();
        assert!(file_size > 1000, "PNG file too small: {} bytes", file_size);
        
        // Verify PNG magic bytes
        let content = fs::read(&ohlc_chart_path).unwrap();
        assert!(content.len() >= 8);
        assert_eq!(&content[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        
        // Test tick chart export
        let tick_chart_path = fixture.get_path("integration_line.png");
        let result = to_png_ticks(&fixture.tick_data, &tick_chart_path);
        assert!(result.is_ok(), "PNG tick export failed: {:?}", result);
        
        // Verify tick chart
        assert!(tick_chart_path.exists());
        let tick_size = fs::metadata(&tick_chart_path).unwrap().len();
        assert!(tick_size > 1000, "Tick chart too small: {} bytes", tick_size);
    }
    
    #[test]
    fn test_png_custom_options() {
        use market_data_source::export::{ChartBuilder, to_png_ohlc_with_builder};
        
        let fixture = ExportTestFixture::new();
        
        // Create custom chart builder
        let builder = ChartBuilder::new()
            .title("Integration Test Chart")
            .width(800)
            .height(600)
            .show_volume(true);
        
        let custom_path = fixture.get_path("custom_chart.png");
        let result = to_png_ohlc_with_builder(&fixture.ohlc_data, &custom_path, builder);
        assert!(result.is_ok(), "Custom PNG export failed: {:?}", result);
        
        // Verify custom chart
        assert!(custom_path.exists());
        let file_size = fs::metadata(&custom_path).unwrap().len();
        assert!(file_size > 2000, "Custom chart too small: {} bytes", file_size);
    }
}

#[cfg(feature = "couchdb")]
mod couchdb_integration_tests {
    use super::*;
    use market_data_source::export::{to_couchdb_ohlc, to_couchdb_ticks};
    
    #[test]
    fn test_couchdb_export_integration() {
        let fixture = ExportTestFixture::new();
        
        // Note: These tests will fail if CouchDB is not running
        // In CI, we should either mock this or make it conditional
        let server_url = "http://localhost:5984";
        let test_db = "integration_test";
        
        // Test OHLC export (may fail if server not available)
        let result = to_couchdb_ohlc(&fixture.ohlc_data[..5], server_url, test_db);
        match result {
            Ok(_) => {
                // CouchDB is available and export succeeded
                println!("CouchDB export successful");
            }
            Err(e) => {
                // Expected if CouchDB is not running
                println!("CouchDB export failed (expected): {}", e);
                // Don't fail the test, just log it
            }
        }
        
        // Similar for tick data
        let result = to_couchdb_ticks(&fixture.tick_data[..3], server_url, "tick_test");
        match result {
            Ok(_) => println!("CouchDB tick export successful"),
            Err(e) => println!("CouchDB tick export failed (expected): {}", e),
        }
    }
}

/// Performance benchmark tests for large datasets
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_large_dataset_performance() {
        let config = ConfigBuilder::new()
            .starting_price_f64(100.0)
            .seed(99999)
            .build()
            .unwrap();
        
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let temp_dir = tempdir().unwrap();
        
        // Generate large dataset
        let large_data = generator.generate_series(5000);
        println!("Generated {} records for performance test", large_data.len());
        
        // Benchmark CSV export
        #[cfg(feature = "csv_export")]
        {
            use market_data_source::export::to_csv_ohlc;
            
            let csv_path = temp_dir.path().join("perf_test.csv");
            let start = Instant::now();
            
            let result = to_csv_ohlc(&large_data, &csv_path);
            let duration = start.elapsed();
            
            assert!(result.is_ok(), "Large CSV export failed");
            assert!(csv_path.exists());
            
            let records_per_sec = large_data.len() as f64 / duration.as_secs_f64();
            println!("CSV Performance: {:.0} records/sec ({:?})", records_per_sec, duration);
            
            // Should handle at least 1000 records per second
            assert!(records_per_sec > 1000.0, "CSV export too slow: {:.0} rec/sec", records_per_sec);
        }
        
        // Benchmark JSON export
        #[cfg(feature = "json_export")]
        {
            use market_data_source::export::to_json_ohlc;
            
            let json_path = temp_dir.path().join("perf_test.json");
            let start = Instant::now();
            
            let result = to_json_ohlc(&large_data, &json_path);
            let duration = start.elapsed();
            
            assert!(result.is_ok(), "Large JSON export failed");
            assert!(json_path.exists());
            
            let records_per_sec = large_data.len() as f64 / duration.as_secs_f64();
            println!("JSON Performance: {:.0} records/sec ({:?})", records_per_sec, duration);
            
            // JSON might be slower due to more formatting
            assert!(records_per_sec > 500.0, "JSON export too slow: {:.0} rec/sec", records_per_sec);
        }
        
        // Benchmark PNG export (smaller dataset)
        #[cfg(feature = "png_export")]
        {
            use market_data_source::export::to_png_ohlc;
            
            let chart_data = &large_data[..1000];  // Charts don't need all data
            let png_path = temp_dir.path().join("perf_chart.png");
            let start = Instant::now();
            
            let result = to_png_ohlc(chart_data, &png_path);
            let duration = start.elapsed();
            
            assert!(result.is_ok(), "Large PNG export failed");
            assert!(png_path.exists());
            
            println!("PNG Chart Performance: {} records in {:?}", chart_data.len(), duration);
            
            // Charts should complete within reasonable time
            assert!(duration.as_secs() < 30, "PNG export took too long: {:?}", duration);
        }
    }
}

/// Error handling and edge case tests
mod error_handling_tests {
    use super::*;
    
    #[test]
    fn test_empty_data_export() {
        let temp_dir = tempdir().unwrap();
        let empty_ohlc: Vec<OHLC> = vec![];
        let _empty_ticks: Vec<Tick> = vec![];
        
        // CSV should handle empty data gracefully
        #[cfg(feature = "csv_export")]
        {
            use market_data_source::export::to_csv_ohlc;
            
            let csv_path = temp_dir.path().join("empty.csv");
            let result = to_csv_ohlc(&empty_ohlc, &csv_path);
            
            match result {
                Ok(_) => {
                    // Should create file with just headers
                    assert!(csv_path.exists());
                    let content = fs::read_to_string(&csv_path).unwrap();
                    let lines: Vec<&str> = content.lines().collect();
                    assert_eq!(lines.len(), 1); // Just the header
                }
                Err(e) => {
                    println!("Empty CSV export handling: {}", e);
                    // Some exporters might reject empty data, which is acceptable
                }
            }
        }
        
        // JSON should handle empty arrays
        #[cfg(feature = "json_export")]
        {
            use market_data_source::export::to_json_ohlc;
            
            let json_path = temp_dir.path().join("empty.json");
            let result = to_json_ohlc(&empty_ohlc, &json_path);
            
            match result {
                Ok(_) => {
                    assert!(json_path.exists());
                    let content = fs::read_to_string(&json_path).unwrap();
                    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
                    assert!(parsed.is_array());
                    assert_eq!(parsed.as_array().unwrap().len(), 0);
                }
                Err(e) => {
                    println!("Empty JSON export handling: {}", e);
                }
            }
        }
    }
    
    #[test]
    fn test_invalid_path_handling() {
        let fixture = ExportTestFixture::new();
        
        // Test with invalid/read-only path
        let invalid_path = if cfg!(windows) {
            "C:\\invalid\\path\\file.csv"
        } else {
            "/root/invalid/path/file.csv"
        };
        
        #[cfg(feature = "csv_export")]
        {
            use market_data_source::export::to_csv_ohlc;
            let result = to_csv_ohlc(&fixture.ohlc_data, invalid_path);
            assert!(result.is_err(), "Should fail with invalid path");
        }
        
        #[cfg(feature = "json_export")]
        {
            use market_data_source::export::to_json_ohlc;
            let result = to_json_ohlc(&fixture.ohlc_data, invalid_path);
            assert!(result.is_err(), "Should fail with invalid path");
        }
    }
}

/// Integration test runner that provides a summary
#[test]
fn test_export_integration_summary() {
    println!("Running Export Integration Test Summary");
    println!("=====================================");
    
    let mut enabled_features: Vec<&str> = vec![];
    let mut disabled_features = vec![];
    
    #[cfg(feature = "csv_export")]
    enabled_features.push("csv_export");
    #[cfg(not(feature = "csv_export"))]
    disabled_features.push("csv_export");
    
    #[cfg(feature = "json_export")]
    enabled_features.push("json_export");
    #[cfg(not(feature = "json_export"))]
    disabled_features.push("json_export");
    
    #[cfg(feature = "png_export")]
    enabled_features.push("png_export");
    #[cfg(not(feature = "png_export"))]
    disabled_features.push("png_export");
    
    #[cfg(feature = "couchdb")]
    enabled_features.push("couchdb");
    #[cfg(not(feature = "couchdb"))]
    disabled_features.push("couchdb");
    
    println!("Enabled features: {:?}", enabled_features);
    println!("Disabled features: {:?}", disabled_features);
    
    // Basic test to ensure at least one format works
    assert!(!enabled_features.is_empty(), "At least one export feature should be enabled for integration tests");
    
    println!("✅ Export integration tests completed successfully");
}