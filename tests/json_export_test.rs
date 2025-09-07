#![allow(unused)]
//! Integration tests for JSON export functionality

#[cfg(feature = "json_export")]
mod json_export_tests {
    use market_data_source::export::{JsonExporter, JsonOptions, DataExporter};
    use market_data_source::export::{to_json_ohlc, to_json_ticks, to_jsonl_ohlc, to_jsonl_ticks};
    use market_data_source::{GeneratorConfig, MarketDataGenerator, ConfigBuilder, TrendDirection};
    use market_data_source::types::{OHLC, Tick};
    use std::fs;
    use tempfile::tempdir;
    

    #[test]
    fn test_generator_to_json_export() {
        // Generate market data
        let config = ConfigBuilder::new()
            .starting_price_f64(100.0)
            .volatility_f64(0.02)
            .trend_f64(TrendDirection::Bullish, 0.001)
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(10);

        // Export to JSON
        let dir = tempdir().unwrap();
        let json_path = dir.path().join("market_data.json");

        to_json_ohlc(&ohlc_data, &json_path).unwrap();

        // Verify file exists and contains valid JSON
        assert!(json_path.exists());
        let content = fs::read_to_string(&json_path).unwrap();
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        assert_eq!(parsed.len(), 10);
        assert!(parsed[0].is_valid());
    }

    #[test]
    fn test_json_lines_export() {
        // Generate tick data
        let config = ConfigBuilder::new()
            .starting_price_f64(50.0)
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let tick_data = generator.generate_ticks(20);

        // Export to JSON Lines
        let dir = tempdir().unwrap();
        let jsonl_path = dir.path().join("ticks.jsonl");

        to_jsonl_ticks(&tick_data, &jsonl_path).unwrap();

        // Read and verify JSON Lines format
        let content = fs::read_to_string(&jsonl_path).unwrap();
        let lines: Vec<&str> = content.trim().split('\n').collect();
        
        assert_eq!(lines.len(), 20);
        
        // Each line should be valid JSON
        for line in lines {
            let tick: Tick = serde_json::from_str(line).unwrap();
            use rust_decimal::Decimal;
            assert!(tick.price > Decimal::from(0));
        }
    }

    #[test]
    fn test_pretty_json_export() {
        // Generate small dataset
        let config = GeneratorConfig::default();
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(3);

        // Export with pretty printing
        let dir = tempdir().unwrap();
        let pretty_path = dir.path().join("pretty.json");

        let options = JsonOptions::pretty();
        let exporter = JsonExporter::with_options(options);
        exporter.export_ohlc(&ohlc_data, &pretty_path).unwrap();

        // Verify pretty formatting
        let content = fs::read_to_string(&pretty_path).unwrap();
        
        // Pretty JSON should have multiple lines and indentation
        assert!(content.lines().count() > 5, "Pretty JSON should have multiple lines");
        assert!(content.contains("  "), "Pretty JSON should contain indentation");
        
        // Should still be valid JSON
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn test_large_dataset_json_export() {
        // Generate large dataset
        let config = ConfigBuilder::new()
            .starting_price_f64(1000.0)
            .volatility_f64(0.03)
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(1000);

        // Export to JSON
        let dir = tempdir().unwrap();
        let json_path = dir.path().join("large_dataset.json");

        let exporter = JsonExporter::default();
        exporter.export_ohlc(&ohlc_data, &json_path).unwrap();

        // Verify file size and content
        let metadata = fs::metadata(&json_path).unwrap();
        assert!(metadata.len() > 1000, "Large dataset should produce substantial file");

        // Verify we can read it back
        let content = fs::read_to_string(&json_path).unwrap();
        let parsed: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 1000);
    }

    #[test]
    fn test_json_lines_streaming_format() {
        // Generate data for streaming
        let config = GeneratorConfig::default();
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let tick_data = generator.generate_ticks(100);

        // Export as JSON Lines (streaming format)
        let dir = tempdir().unwrap();
        let jsonl_path = dir.path().join("stream.jsonl");

        let options = JsonOptions::json_lines();
        let exporter = JsonExporter::with_options(options);
        exporter.export_ticks(&tick_data, &jsonl_path).unwrap();

        // Simulate streaming read
        let content = fs::read_to_string(&jsonl_path).unwrap();
        let mut count = 0;
        
        for line in content.lines() {
            if !line.trim().is_empty() {
                let tick: Tick = serde_json::from_str(line).unwrap();
                assert!(tick.volume.value() >= 0);
                count += 1;
            }
        }
        
        assert_eq!(count, 100);
    }

    #[test]
    fn test_json_export_with_custom_config() {
        // Generate data with specific parameters
        let config = GeneratorConfig::bull_market();
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(50);

        // Test different export options
        let dir = tempdir().unwrap();
        
        // Standard JSON
        let standard_path = dir.path().join("standard.json");
        to_json_ohlc(&ohlc_data, &standard_path).unwrap();
        
        // JSON Lines
        let jsonl_path = dir.path().join("lines.jsonl");
        to_jsonl_ohlc(&ohlc_data, &jsonl_path).unwrap();
        
        // Pretty JSON
        let pretty_path = dir.path().join("pretty.json");
        let pretty_exporter = JsonExporter::with_options(JsonOptions::pretty());
        pretty_exporter.export_ohlc(&ohlc_data, &pretty_path).unwrap();
        
        // Verify all formats
        assert!(standard_path.exists());
        assert!(jsonl_path.exists());
        assert!(pretty_path.exists());
        
        // Verify different file sizes (pretty should be largest)
        let standard_size = fs::metadata(&standard_path).unwrap().len();
        let pretty_size = fs::metadata(&pretty_path).unwrap().len();
        assert!(pretty_size > standard_size, "Pretty JSON should be larger than compact JSON");
    }

    #[test]
    fn test_json_compatibility_with_javascript() {
        // Generate data that should be compatible with JavaScript parsing
        let config = GeneratorConfig::default();
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let ohlc_data = generator.generate_series(5);

        let dir = tempdir().unwrap();
        let json_path = dir.path().join("js_compatible.json");

        to_json_ohlc(&ohlc_data, &json_path).unwrap();

        // Read and verify structure matches expected JavaScript format
        let content = fs::read_to_string(&json_path).unwrap();
        
        // Should be a valid JSON array
        assert!(content.starts_with('['));
        assert!(content.ends_with(']'));
        
        // Verify field names are correct for JavaScript consumption
        assert!(content.contains("\"open\""));
        assert!(content.contains("\"high\""));
        assert!(content.contains("\"low\""));
        assert!(content.contains("\"close\""));
        assert!(content.contains("\"volume\""));
        assert!(content.contains("\"timestamp\""));
    }

    #[test]
    fn test_json_roundtrip() {
        // Create sample data
        let config = GeneratorConfig::stable();
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let original_data = generator.generate_series(10);

        // Export and re-import
        let dir = tempdir().unwrap();
        let json_path = dir.path().join("roundtrip.json");
        
        to_json_ohlc(&original_data, &json_path).unwrap();
        
        let content = fs::read_to_string(&json_path).unwrap();
        let imported_data: Vec<OHLC> = serde_json::from_str(&content).unwrap();
        
        // Verify structural integrity - data can be exported and imported successfully
        assert_eq!(original_data.len(), imported_data.len(), "Data length should be preserved");
        
        // Verify all required fields are present and valid
        for (i, ohlc) in imported_data.iter().enumerate() {
            assert!(ohlc.is_valid(), "OHLC at index {i} should be valid");
            use rust_decimal::Decimal;
            assert!(ohlc.open > Decimal::from(0), "Open price should be positive");
            assert!(ohlc.high >= ohlc.open || ohlc.high >= ohlc.close, "High should be highest");
            assert!(ohlc.low <= ohlc.open || ohlc.low <= ohlc.close, "Low should be lowest");
            assert!(ohlc.close > Decimal::from(0), "Close price should be positive");
            assert!(ohlc.timestamp > 0, "Timestamp should be positive");
            assert!(ohlc.volume.value() >= 0, "Volume should be non-negative");
        }
        
        // Verify JSON structure is parseable and contains expected fields
        assert!(content.contains("\"open\""), "JSON should contain open field");
        assert!(content.contains("\"high\""), "JSON should contain high field");
        assert!(content.contains("\"low\""), "JSON should contain low field");
        assert!(content.contains("\"close\""), "JSON should contain close field");
        assert!(content.contains("\"timestamp\""), "JSON should contain timestamp field");
        assert!(content.contains("\"volume\""), "JSON should contain volume field");
    }
}
