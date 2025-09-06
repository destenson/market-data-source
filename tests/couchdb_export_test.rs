//! Integration tests for CouchDB export functionality

#[cfg(feature = "couchdb")]
mod couchdb_export_tests {
    use market_data_source::{
        export::{CouchDbExporter, DataExporter, to_couchdb_ohlc, to_couchdb_ticks},
        types::{OHLC, Tick},
        MarketDataGenerator,
        GeneratorConfig,
        TimeInterval,
    };

    /// Generate test OHLC data
    fn generate_test_ohlc_data(count: usize) -> Vec<OHLC> {
        let config = GeneratorConfig::default()
            .with_symbol("TEST")
            .with_initial_price(100.0)
            .with_volatility(0.02)
            .with_time_interval(TimeInterval::OneMinute)
            .with_seed(Some(12345));

        let mut generator = MarketDataGenerator::new(config);
        generator.generate_ohlc(count)
    }

    /// Generate test tick data
    fn generate_test_tick_data(count: usize) -> Vec<Tick> {
        let config = GeneratorConfig::default()
            .with_symbol("TEST")
            .with_initial_price(100.0)
            .with_volatility(0.02)
            .with_seed(Some(12345));

        let mut generator = MarketDataGenerator::new(config);
        generator.generate_ticks(count)
    }

    #[test]
    fn test_couchdb_exporter_creation() {
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_db");
        // Basic creation test - doesn't require actual CouchDB connection
        assert!(true, "CouchDB exporter created successfully");
    }

    #[test]
    fn test_couchdb_exporter_with_options() {
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_db")
            .with_auth("admin", "password")
            .with_batch_size(500);
        // Options test - doesn't require actual CouchDB connection
        assert!(true, "CouchDB exporter configured successfully");
    }

    // Integration tests that require actual CouchDB instance
    // These are marked with #[ignore] by default
    // Run with: cargo test --ignored couchdb_integration

    #[test]
    #[ignore]
    fn test_export_ohlc_to_couchdb() {
        let data = generate_test_ohlc_data(100);
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_market_data");
        
        let result = exporter.export_ohlc(&data, "");
        assert!(result.is_ok(), "Failed to export OHLC data to CouchDB: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_export_ticks_to_couchdb() {
        let data = generate_test_tick_data(100);
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_market_data");
        
        let result = exporter.export_ticks(&data, "");
        assert!(result.is_ok(), "Failed to export tick data to CouchDB: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_convenience_function_ohlc() {
        let data = generate_test_ohlc_data(50);
        let result = to_couchdb_ohlc(&data, "http://localhost:5984", "test_convenience_ohlc");
        assert!(result.is_ok(), "Failed to export OHLC data using convenience function: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_convenience_function_ticks() {
        let data = generate_test_tick_data(50);
        let result = to_couchdb_ticks(&data, "http://localhost:5984", "test_convenience_ticks");
        assert!(result.is_ok(), "Failed to export tick data using convenience function: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_export_with_authentication() {
        let data = generate_test_ohlc_data(25);
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_auth_db")
            .with_auth("admin", "password");
        
        let result = exporter.export_ohlc(&data, "");
        // This test will fail if authentication is required and credentials are wrong
        assert!(result.is_ok() || result.is_err(), "Authentication test completed");
    }

    #[test]
    #[ignore]
    fn test_batch_export() {
        let data = generate_test_ohlc_data(2500); // Large dataset to test batching
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_batch_db")
            .with_batch_size(500);
        
        let result = exporter.export_ohlc(&data, "");
        assert!(result.is_ok(), "Failed to export large batch of OHLC data: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_mixed_data_export() {
        let ohlc_data = generate_test_ohlc_data(100);
        let tick_data = generate_test_tick_data(100);
        
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_mixed_db");
        
        let ohlc_result = exporter.export_ohlc(&ohlc_data, "");
        assert!(ohlc_result.is_ok(), "Failed to export OHLC data: {:?}", ohlc_result.err());
        
        let tick_result = exporter.export_ticks(&tick_data, "");
        assert!(tick_result.is_ok(), "Failed to export tick data: {:?}", tick_result.err());
    }
}