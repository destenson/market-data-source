//! JSON Export Example
//!
//! This example demonstrates how to export market data to JSON formats.
//! It covers both standard JSON and JSON Lines (JSONL) formats for OHLC and tick data.

use market_data_source::{
    MarketDataGenerator, ConfigBuilder, TrendDirection,
    export::{to_json_ohlc, to_json_ticks, to_jsonl_ohlc, to_jsonl_ticks}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Market Data Source - JSON Export Example");
    println!("========================================");
    
    // Create a generator with custom configuration
    let config = ConfigBuilder::new()
        .starting_price(200.0)
        .volatility(0.03)
        .trend(TrendDirection::Sideways, 0.0)
        .seed(456)  // For reproducible results
        .build()?;
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    
    // Example 1: Standard JSON export for OHLC data
    println!("\n1. Exporting OHLC data to standard JSON...");
    
    // Generate 20 OHLC candles
    let ohlc_data = generator.generate_series(20);
    println!("   Generated {} OHLC candles", ohlc_data.len());
    
    // Export to standard JSON format
    let json_ohlc_file = "market_data_ohlc.json";
    to_json_ohlc(&ohlc_data, json_ohlc_file)?;
    println!("   ✓ Exported OHLC data to standard JSON: {}", json_ohlc_file);
    
    // Example 2: Standard JSON export for tick data
    println!("\n2. Exporting tick data to standard JSON...");
    
    // Generate 15 ticks
    let tick_data = generator.generate_ticks(15);
    println!("   Generated {} ticks", tick_data.len());
    
    // Export to standard JSON format
    let json_tick_file = "market_data_ticks.json";
    to_json_ticks(&tick_data, json_tick_file)?;
    println!("   ✓ Exported tick data to standard JSON: {}", json_tick_file);
    
    // Example 3: JSON Lines (JSONL) export for OHLC data
    println!("\n3. Exporting OHLC data to JSON Lines format...");
    
    // Generate fresh data for JSONL
    let jsonl_ohlc_data = generator.generate_series(10);
    
    // Export to JSON Lines format (one JSON object per line)
    let jsonl_ohlc_file = "market_data_ohlc.jsonl";
    to_jsonl_ohlc(&jsonl_ohlc_data, jsonl_ohlc_file)?;
    println!("   ✓ Exported OHLC data to JSON Lines: {}", jsonl_ohlc_file);
    println!("   JSON Lines format is ideal for streaming and big data processing");
    
    // Example 4: JSON Lines (JSONL) export for tick data
    println!("\n4. Exporting tick data to JSON Lines format...");
    
    // Generate fresh tick data for JSONL
    let jsonl_tick_data = generator.generate_ticks(10);
    
    // Export to JSON Lines format
    let jsonl_tick_file = "market_data_ticks.jsonl";
    to_jsonl_ticks(&jsonl_tick_data, jsonl_tick_file)?;
    println!("   ✓ Exported tick data to JSON Lines: {}", jsonl_tick_file);
    
    // Example 5: Custom JSON export options
    println!("\n5. Custom JSON export with pretty printing...");
    
    use market_data_source::export::json::{JsonExporter, JsonOptions};
    use market_data_source::export::DataExporter;
    
    // Create pretty-printed JSON exporter
    let pretty_options = JsonOptions::pretty();
    let pretty_exporter = JsonExporter::with_options(pretty_options);
    
    // Generate small dataset for pretty printing
    let pretty_data = generator.generate_series(5);
    let pretty_file = "pretty_formatted.json";
    pretty_exporter.export_ohlc(&pretty_data, pretty_file)?;
    println!("   ✓ Exported with pretty formatting to: {}", pretty_file);
    
    // Example 6: Compact JSON (no pretty printing)
    println!("\n6. Compact JSON export...");
    
    let compact_options = JsonOptions::default(); // Compact by default (no pretty printing)
    let compact_exporter = JsonExporter::with_options(compact_options);
    
    let compact_file = "compact_formatted.json";
    compact_exporter.export_ohlc(&pretty_data, compact_file)?;
    println!("   ✓ Exported in compact format to: {}", compact_file);
    
    // Example 7: Reading and verifying JSON data
    println!("\n7. Verifying exported JSON data...");
    
    // Read back the JSON file to verify it's valid
    #[cfg(feature = "json_export")]
    {
        let json_content = std::fs::read_to_string(json_ohlc_file)?;
        let parsed: serde_json::Value = serde_json::from_str(&json_content)?;
        
        if let Some(array) = parsed.as_array() {
            println!("   ✓ Successfully parsed JSON file with {} records", array.len());
            
            // Show first record structure
            if let Some(first_record) = array.first() {
                println!("   Sample record structure: {}", 
                    serde_json::to_string_pretty(first_record)?);
            }
        }
    }
    
    // Example 8: Performance comparison
    println!("\n8. Performance test - generating larger dataset...");
    
    // Create fresh generator for performance test
    let perf_config = ConfigBuilder::new()
        .starting_price(100.0)
        .seed(789)
        .build()?;
    
    let mut perf_generator = MarketDataGenerator::with_config(perf_config)?;
    
    // Generate larger dataset
    let large_data = perf_generator.generate_series(1000);
    
    // Time JSON export
    let start = std::time::Instant::now();
    let large_json_file = "large_dataset.json";
    to_json_ohlc(&large_data, large_json_file)?;
    let json_duration = start.elapsed();
    
    // Time JSONL export
    let start = std::time::Instant::now();
    let large_jsonl_file = "large_dataset.jsonl";
    to_jsonl_ohlc(&large_data, large_jsonl_file)?;
    let jsonl_duration = start.elapsed();
    
    println!("   Standard JSON export: {:?} for {} records", json_duration, large_data.len());
    println!("   JSON Lines export: {:?} for {} records", jsonl_duration, large_data.len());
    
    println!("\n✅ JSON export examples completed successfully!");
    println!("\nGenerated files:");
    println!("  - {} (Standard JSON - OHLC)", json_ohlc_file);
    println!("  - {} (Standard JSON - Ticks)", json_tick_file);
    println!("  - {} (JSON Lines - OHLC)", jsonl_ohlc_file);
    println!("  - {} (JSON Lines - Ticks)", jsonl_tick_file);
    println!("  - {} (Pretty formatted)", pretty_file);
    println!("  - {} (Compact formatted)", compact_file);
    println!("  - {} (Large dataset - JSON)", large_json_file);
    println!("  - {} (Large dataset - JSONL)", large_jsonl_file);
    
    println!("\nFormat comparison:");
    println!("  📄 Standard JSON: Complete JSON array, good for small-medium datasets");
    println!("  📝 JSON Lines: One JSON object per line, excellent for streaming & big data");
    println!("  🎨 Pretty JSON: Human-readable with formatting");
    println!("  📦 Compact JSON: Minimal size, machine-optimized");
    
    println!("\nUsage tips:");
    println!("  - Use JSON Lines (.jsonl) for large datasets and streaming");
    println!("  - Use standard JSON (.json) for API responses and small datasets");
    println!("  - Use pretty formatting for debugging and human review");
    println!("  - Use compact formatting for production and minimal bandwidth");
    
    Ok(())
}