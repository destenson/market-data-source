#![allow(unused)]
//! CouchDB Export Example
//!
//! This example demonstrates how to export market data to CouchDB database.
//! It covers both manual configuration and environment variable setup.
//!
//! Note: This example requires a running CouchDB instance and the 'couchdb' feature.

#[cfg(feature = "couchdb")]
use market_data_source::{
    export::{to_couchdb_ohlc, to_couchdb_ticks},
    ConfigBuilder, MarketDataGenerator, TrendDirection,
};

#[cfg(all(feature = "couchdb", feature = "dotenvy"))]
use market_data_source::export::{to_couchdb_ohlc_env, to_couchdb_ticks_env};

#[cfg(feature = "couchdb")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Market Data Source - CouchDB Export Example");
    println!("===========================================");

    // Check if CouchDB server is likely available
    println!("\n⚠️  Prerequisites:");
    println!("   1. CouchDB server running (default: http://localhost:5984)");
    println!("   2. Database created (or allow auto-creation)");
    println!("   3. Proper authentication configured");
    println!("   4. Network connectivity to CouchDB server");

    // Create a generator with sample data
    let config = ConfigBuilder::new()
        .starting_price_f64(180.0)
        .volatility_f64(0.02)
        .trend_f64(TrendDirection::Bullish, 0.001)
        .seed(654)
        .build()?;

    let mut generator = MarketDataGenerator::with_config(config)?;

    // Example 1: Basic CouchDB export with manual configuration
    println!("\n1. Exporting OHLC data to CouchDB with manual config...");

    let ohlc_data = generator.generate_series(25);
    println!("   Generated {} OHLC candles", ohlc_data.len());

    // Configuration for local CouchDB instance
    let server_url = "http://localhost:5984";
    let database_name = "market_data_ohlc";

    match to_couchdb_ohlc(&ohlc_data, server_url, database_name) {
        Ok(_) => {
            println!("   ✓ Successfully exported OHLC data to CouchDB");
            println!("   Database: {server_url}/{database_name}");
        }
        Err(e) => {
            println!("   ❌ Failed to export to CouchDB: {e}");
            println!("   This is expected if CouchDB is not running or configured");
        }
    }

    // Example 2: Tick data export to CouchDB
    println!("\n2. Exporting tick data to CouchDB...");

    let tick_data = generator.generate_ticks(30);
    println!("   Generated {} ticks", tick_data.len());

    let tick_database = "market_data_ticks";

    match to_couchdb_ticks(&tick_data, server_url, tick_database) {
        Ok(_) => {
            println!("   ✓ Successfully exported tick data to CouchDB");
            println!("   Database: {server_url}/{tick_database}");
        }
        Err(e) => {
            println!("   ❌ Failed to export tick data: {e}");
        }
    }

    // Example 3: Using CouchDB exporter directly with custom options
    println!("\n3. Using CouchDB exporter with custom configuration...");

    use market_data_source::export::couchdb::{CouchDbExporter, CouchDbOptions};
    use market_data_source::export::DataExporter;

    // Create custom CouchDB options
    let options = CouchDbOptions::new()
        .timeout_seconds(30)
        .batch_size(100)
        .auto_create_database(true)
        .username("admin")
        .password("admin");

    let exporter = CouchDbExporter::new_with_options(server_url, "custom_market_data", options);

    let custom_data = generator.generate_series(15);

    match exporter.export_ohlc(&custom_data, "") {
        Ok(_) => {
            println!("   ✓ Successfully exported with custom options");
            println!("   Database: {server_url}/custom_market_data");
        }
        Err(e) => {
            println!("   ❌ Custom export failed: {e}");
        }
    }

    // Example 4: Environment variable configuration (if dotenvy feature enabled)
    #[cfg(feature = "dotenvy")]
    {
        println!("\n4. Using environment variables for configuration...");
        println!("   Set these environment variables for automatic config:");
        println!("   - COUCHDB_URL=http://localhost:5984");
        println!("   - COUCHDB_USERNAME=admin");
        println!("   - COUCHDB_PASSWORD=admin");
        println!("   - COUCHDB_DATABASE=market_data");

        match to_couchdb_ohlc_env(&ohlc_data) {
            Ok(_) => {
                println!("   ✓ Successfully exported using environment config");
            }
            Err(e) => {
                println!("   ❌ Environment-based export failed: {e}");
                println!("   Make sure environment variables are set correctly");
            }
        }

        match to_couchdb_ticks_env(&tick_data) {
            Ok(_) => {
                println!("   ✓ Successfully exported tick data using environment config");
            }
            Err(e) => {
                println!("   ❌ Tick data environment export failed: {e}");
            }
        }
    }

    #[cfg(not(feature = "dotenvy"))]
    {
        println!("\n4. Environment variable configuration (feature not enabled)");
        println!(
            "   Enable with: cargo run --example export_couchdb --features \"couchdb,dotenvy\""
        );
    }

    // Example 5: Batch export for large datasets
    println!("\n5. Batch export for large datasets...");

    let large_data = generator.generate_series(200);
    println!("   Generated {} records for batch export", large_data.len());

    let batch_options = CouchDbOptions::new()
        .batch_size(50) // Export in batches of 50
        .timeout_seconds(60);

    let batch_exporter =
        CouchDbExporter::new_with_options(server_url, "large_dataset", batch_options);

    match batch_exporter.export_ohlc(&large_data, "") {
        Ok(_) => {
            println!("   ✓ Successfully exported large dataset in batches");
        }
        Err(e) => {
            println!("   ❌ Batch export failed: {e}");
        }
    }

    // Example 6: Error handling and retry logic
    println!("\n6. Demonstrating error handling...");

    // Try to export to invalid server (should fail)
    let invalid_server = "http://invalid-server:5984";
    let invalid_database = "test_db";

    println!("   Attempting export to invalid server (should fail)...");
    match to_couchdb_ohlc(&ohlc_data[..5], invalid_server, invalid_database) {
        Ok(_) => {
            println!("   Unexpected success with invalid server");
        }
        Err(e) => {
            println!("   ✓ Properly handled connection error: {e}");
        }
    }

    println!("\n✅ CouchDB export examples completed!");
    println!("\n📋 Summary:");
    println!("   - Manual server configuration");
    println!("   - Environment variable configuration");
    println!("   - Custom export options");
    println!("   - Batch processing for large datasets");
    println!("   - Proper error handling");

    println!("\n🔧 Setup Instructions:");
    println!("   1. Install CouchDB: https://couchdb.apache.org/");
    println!("   2. Start CouchDB service");
    println!("   3. Access Fauxton UI: http://localhost:5984/_utils/");
    println!("   4. Create admin user if needed");
    println!("   5. Optionally create databases manually");

    println!("\n💡 Production Tips:");
    println!("   - Use authentication in production");
    println!("   - Configure proper network security");
    println!("   - Monitor database size and performance");
    println!("   - Use batch exports for large datasets");
    println!("   - Implement retry logic for network failures");

    println!("\n🔍 Verify Data:");
    println!("   Open Fauxton UI to browse exported documents");
    println!("   Use CouchDB HTTP API to query data");
    println!("   Example: curl http://localhost:5984/market_data_ohlc/_all_docs");

    Ok(())
}

#[cfg(not(feature = "couchdb"))]
fn main() {
    println!("CouchDB export feature is not enabled.");
    println!("Run with: cargo run --example export_couchdb --features couchdb");
    println!("Or with environment support: cargo run --example export_couchdb --features \"couchdb,dotenvy\"");

    println!("\n📋 This example would demonstrate:");
    println!("   - Exporting OHLC and tick data to CouchDB");
    println!("   - Manual server configuration");
    println!("   - Environment variable configuration");
    println!("   - Custom export options and batching");
    println!("   - Error handling and connection management");

    std::process::exit(1);
}
