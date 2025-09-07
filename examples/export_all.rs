#![allow(unused)]
//! All Export Formats Example
//!
//! This comprehensive example demonstrates how to export the same market data
//! to all supported formats: CSV, JSON, CouchDB, and PNG charts.
//! It showcases the unified API and format-specific features.

use market_data_source::{
    MarketDataGenerator, ConfigBuilder, TrendDirection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Market Data Source - All Export Formats Example");
    println!("===============================================");
    
    // Create a generator with interesting data for demonstration
    let config = ConfigBuilder::new()
        .starting_price_f64(250.0)
        .volatility_f64(0.03)
        .trend_f64(TrendDirection::Bullish, 0.001)
        .seed(777)  // Lucky number for reproducible results
        .build()?;
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    
    // Generate the same dataset for all export formats
    println!("\n📊 Generating market data...");
    let ohlc_data = generator.generate_series(100);
    let tick_data = generator.generate_ticks(50);
    
    println!("   ✓ Generated {} OHLC candles", ohlc_data.len());
    println!("   ✓ Generated {} ticks", tick_data.len());
    
    // Track successful exports
    let mut successful_exports: Vec<&str> = Vec::new();
    let mut failed_exports: Vec<&str> = Vec::new();
    
    // Export 1: CSV Format
    println!("\n📄 Exporting to CSV format...");
    
    #[cfg(feature = "csv_export")]
    {
        use market_data_source::export::{to_csv_ohlc, to_csv_ticks};
        
        // Export OHLC to CSV
        match to_csv_ohlc(&ohlc_data, "all_formats_ohlc.csv") {
            Ok(_) => {
                println!("   ✅ OHLC data exported to CSV");
                successful_exports.push("CSV OHLC");
            }
            Err(e) => {
                println!("   ❌ CSV OHLC export failed: {}", e);
                failed_exports.push("CSV OHLC");
            }
        }
        
        // Export ticks to CSV
        match to_csv_ticks(&tick_data, "all_formats_ticks.csv") {
            Ok(_) => {
                println!("   ✅ Tick data exported to CSV");
                successful_exports.push("CSV Ticks");
            }
            Err(e) => {
                println!("   ❌ CSV Ticks export failed: {}", e);
                failed_exports.push("CSV Ticks");
            }
        }
    }
    
    #[cfg(not(feature = "csv_export"))]
    {
        println!("   ⚠️  CSV export feature not enabled");
        failed_exports.push("CSV (feature disabled)");
    }
    
    // Export 2: JSON Format
    println!("\n🗂️ Exporting to JSON format...");
    
    #[cfg(feature = "json_export")]
    {
        use market_data_source::export::{
            to_json_ohlc, to_json_ticks, 
            to_jsonl_ohlc, to_jsonl_ticks
        };
        
        // Standard JSON exports
        match to_json_ohlc(&ohlc_data, "all_formats_ohlc.json") {
            Ok(_) => {
                println!("   ✅ OHLC data exported to JSON");
                successful_exports.push("JSON OHLC");
            }
            Err(e) => {
                println!("   ❌ JSON OHLC export failed: {}", e);
                failed_exports.push("JSON OHLC");
            }
        }
        
        match to_json_ticks(&tick_data, "all_formats_ticks.json") {
            Ok(_) => {
                println!("   ✅ Tick data exported to JSON");
                successful_exports.push("JSON Ticks");
            }
            Err(e) => {
                println!("   ❌ JSON Ticks export failed: {}", e);
                failed_exports.push("JSON Ticks");
            }
        }
        
        // JSON Lines exports
        match to_jsonl_ohlc(&ohlc_data, "all_formats_ohlc.jsonl") {
            Ok(_) => {
                println!("   ✅ OHLC data exported to JSON Lines");
                successful_exports.push("JSONL OHLC");
            }
            Err(e) => {
                println!("   ❌ JSON Lines OHLC export failed: {}", e);
                failed_exports.push("JSONL OHLC");
            }
        }
        
        match to_jsonl_ticks(&tick_data, "all_formats_ticks.jsonl") {
            Ok(_) => {
                println!("   ✅ Tick data exported to JSON Lines");
                successful_exports.push("JSONL Ticks");
            }
            Err(e) => {
                println!("   ❌ JSON Lines Ticks export failed: {}", e);
                failed_exports.push("JSONL Ticks");
            }
        }
    }
    
    #[cfg(not(feature = "json_export"))]
    {
        println!("   ⚠️  JSON export feature not enabled");
        failed_exports.push("JSON (feature disabled)");
    }
    
    // Export 3: PNG Charts
    println!("\n📊 Exporting to PNG charts...");
    
    #[cfg(feature = "png_export")]
    {
        use market_data_source::export::{
            to_png_ohlc, to_png_ticks, 
            to_png_ohlc_with_builder, ChartBuilder
        };
        
        // Basic candlestick chart
        match to_png_ohlc(&ohlc_data, "all_formats_candlestick.png") {
            Ok(_) => {
                println!("   ✅ OHLC candlestick chart created");
                successful_exports.push("PNG Candlestick");
            }
            Err(e) => {
                println!("   ❌ PNG candlestick export failed: {}", e);
                failed_exports.push("PNG Candlestick");
            }
        }
        
        // Tick line chart
        match to_png_ticks(&tick_data, "all_formats_line_chart.png") {
            Ok(_) => {
                println!("   ✅ Tick line chart created");
                successful_exports.push("PNG Line Chart");
            }
            Err(e) => {
                println!("   ❌ PNG line chart export failed: {}", e);
                failed_exports.push("PNG Line Chart");
            }
        }
        
        // Advanced chart with volume and moving average
        let advanced_builder = ChartBuilder::new()
            .title("All Formats Demo - Advanced Chart")
            .width(1200)
            .height(800)
            .show_volume(true)
            .show_sma(20)
            .background_color((245, 245, 245));
        
        match to_png_ohlc_with_builder(&ohlc_data, "all_formats_advanced.png", advanced_builder) {
            Ok(_) => {
                println!("   ✅ Advanced chart with volume & SMA created");
                successful_exports.push("PNG Advanced Chart");
            }
            Err(e) => {
                println!("   ❌ PNG advanced chart export failed: {}", e);
                failed_exports.push("PNG Advanced Chart");
            }
        }
    }
    
    #[cfg(not(feature = "png_export"))]
    {
        println!("   ⚠️  PNG export feature not enabled");
        failed_exports.push("PNG (feature disabled)");
    }
    
    // Export 4: CouchDB Database
    println!("\n🗄️ Exporting to CouchDB...");
    
    #[cfg(feature = "couchdb")]
    {
        use market_data_source::export::{to_couchdb_ohlc, to_couchdb_ticks};
        
        let server_url = "http://localhost:5984";
        let ohlc_db = "all_formats_ohlc";
        let tick_db = "all_formats_ticks";
        
        // OHLC to CouchDB
        match to_couchdb_ohlc(&ohlc_data, server_url, ohlc_db) {
            Ok(_) => {
                println!("   ✅ OHLC data exported to CouchDB");
                successful_exports.push("CouchDB OHLC");
            }
            Err(e) => {
                println!("   ❌ CouchDB OHLC export failed: {}", e);
                println!("      (This is expected if CouchDB is not running)");
                failed_exports.push("CouchDB OHLC");
            }
        }
        
        // Ticks to CouchDB
        match to_couchdb_ticks(&tick_data, server_url, tick_db) {
            Ok(_) => {
                println!("   ✅ Tick data exported to CouchDB");
                successful_exports.push("CouchDB Ticks");
            }
            Err(e) => {
                println!("   ❌ CouchDB Ticks export failed: {}", e);
                failed_exports.push("CouchDB Ticks");
            }
        }
    }
    
    #[cfg(not(feature = "couchdb"))]
    {
        println!("   ⚠️  CouchDB export feature not enabled");
        failed_exports.push("CouchDB (feature disabled)");
    }
    
    // Export 5: Demonstrate format-specific features
    println!("\n⚙️ Demonstrating format-specific features...");
    
    // CSV with custom delimiter
    #[cfg(feature = "csv_export")]
    {
        use market_data_source::export::csv::CsvExporter;
        use market_data_source::export::DataExporter;
        
        let pipe_exporter = CsvExporter::new().delimiter(b'|');
        match pipe_exporter.export_ohlc(&ohlc_data[..10], "pipe_delimited.csv") {
            Ok(_) => {
                println!("   ✅ Created pipe-delimited CSV sample");
                successful_exports.push("Custom CSV");
            }
            Err(e) => {
                println!("   ❌ Custom CSV failed: {}", e);
                failed_exports.push("Custom CSV");
            }
        }
    }
    
    // Pretty JSON
    #[cfg(feature = "json_export")]
    {
        use market_data_source::export::json::{JsonExporter, JsonOptions};
        use market_data_source::export::DataExporter;
        
        let pretty_exporter = JsonExporter::with_options(JsonOptions::pretty());
        match pretty_exporter.export_ohlc(&ohlc_data[..5], "pretty_sample.json") {
            Ok(_) => {
                println!("   ✅ Created pretty-formatted JSON sample");
                successful_exports.push("Pretty JSON");
            }
            Err(e) => {
                println!("   ❌ Pretty JSON failed: {}", e);
                failed_exports.push("Pretty JSON");
            }
        }
    }
    
    // Performance comparison
    println!("\n⏱️ Performance comparison across formats...");
    
    let perf_data = generator.generate_series(500);
    let mut timings: Vec<(&str, std::time::Duration)> = Vec::new();
    
    // Time CSV export
    #[cfg(feature = "csv_export")]
    {
        use market_data_source::export::to_csv_ohlc;
        let start = std::time::Instant::now();
        let _ = to_csv_ohlc(&perf_data, "perf_test.csv");
        let duration = start.elapsed();
        timings.push(("CSV", duration));
        println!("   CSV export: {:?} for {} records", duration, perf_data.len());
    }
    
    // Time JSON export
    #[cfg(feature = "json_export")]
    {
        use market_data_source::export::to_json_ohlc;
        let start = std::time::Instant::now();
        let _ = to_json_ohlc(&perf_data, "perf_test.json");
        let duration = start.elapsed();
        timings.push(("JSON", duration));
        println!("   JSON export: {:?} for {} records", duration, perf_data.len());
    }
    
    // Results Summary
    println!("\n🎯 Export Results Summary");
    println!("========================");
    
    if !successful_exports.is_empty() {
        println!("\n✅ Successful Exports ({}):", successful_exports.len());
        for export in &successful_exports {
            println!("   - {}", export);
        }
    }
    
    if !failed_exports.is_empty() {
        println!("\n❌ Failed/Disabled Exports ({}):", failed_exports.len());
        for export in &failed_exports {
            println!("   - {}", export);
        }
    }
    
    if !timings.is_empty() {
        println!("\n⏱️ Performance Results:");
        for (format, duration) in &timings {
            println!("   {} export: {:?}", format, duration);
        }
    }
    
    // Generated files list
    println!("\n📁 Generated Files:");
    let potential_files = vec![
        "all_formats_ohlc.csv",
        "all_formats_ticks.csv", 
        "all_formats_ohlc.json",
        "all_formats_ticks.json",
        "all_formats_ohlc.jsonl",
        "all_formats_ticks.jsonl",
        "all_formats_candlestick.png",
        "all_formats_line_chart.png",
        "all_formats_advanced.png",
        "pipe_delimited.csv",
        "pretty_sample.json",
        "perf_test.csv",
        "perf_test.json",
    ];
    
    for file in &potential_files {
        if std::path::Path::new(file).exists() {
            println!("   ✓ {}", file);
        }
    }
    
    println!("\n📊 Format Comparison:");
    println!("   📄 CSV: Universal compatibility, Excel/spreadsheet ready");
    println!("   🗂️ JSON: Web APIs, JavaScript integration, structured data");
    println!("   📝 JSONL: Big data, streaming, line-by-line processing");
    println!("   📊 PNG: Visual analysis, presentations, quick insights");
    println!("   🗄️ CouchDB: Scalable storage, real-time sync, web apps");
    
    println!("\n💡 Usage Recommendations:");
    println!("   - Use CSV for Excel analysis and data science");
    println!("   - Use JSON for web APIs and JavaScript applications");  
    println!("   - Use JSON Lines for big data and streaming pipelines");
    println!("   - Use PNG for visual analysis and presentations");
    println!("   - Use CouchDB for web applications needing real-time data");
    
    println!("\n🚀 Enable All Features:");
    println!("   cargo run --example export_all --all-features");
    
    println!("\n✅ All export formats demonstration completed!");
    
    Ok(())
}
