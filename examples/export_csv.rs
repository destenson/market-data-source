#![allow(unused)]
//! CSV Export Example
//!
//! This example demonstrates how to export market data to CSV format.
//! It covers both OHLC candles and tick data, with different configuration options.

use market_data_source::{MarketDataGenerator, ConfigBuilder, TrendDirection};

#[cfg(feature = "csv_export")]
use market_data_source::export::{to_csv_ohlc, to_csv_ticks};

#[cfg(not(feature = "csv_export"))]
fn main() {
    eprintln!("Error: 'csv_export' feature is not enabled. Please enable it in Cargo.toml to run this example.");
}

#[cfg(feature = "csv_export")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Market Data Source - CSV Export Example");
    println!("=======================================");
    
    // Create a generator with custom configuration
    let config = ConfigBuilder::new()
        .starting_price_f64(150.0)
        .volatility_f64(0.025)
        .trend_f64(TrendDirection::Bullish, 0.001)
        .seed(42)  // For reproducible results
        .build()?;
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    
    // Example 1: Export OHLC data
    println!("\n1. Generating and exporting OHLC data...");
    
    // Generate 100 OHLC candles
    let ohlc_data = generator.generate_series(100);
    println!("   Generated {} OHLC candles", ohlc_data.len());
    
    // Export to CSV file
    let ohlc_file = "market_data_ohlc.csv";
    to_csv_ohlc(&ohlc_data, ohlc_file)?;
    println!("   ✓ Exported OHLC data to: {ohlc_file}");
    
    // Show sample of the data
    println!("   Sample data (first 3 candles):");
    for (i, candle) in ohlc_data.iter().take(3).enumerate() {
        println!("     {}. {}", i + 1, format_ohlc(candle));
    }
    
    // Example 2: Export tick data
    println!("\n2. Generating and exporting tick data...");
    
    // Generate 50 ticks
    let tick_data = generator.generate_ticks(50);
    println!("   Generated {} ticks", tick_data.len());
    
    // Export to CSV file
    let tick_file = "market_data_ticks.csv";
    to_csv_ticks(&tick_data, tick_file)?;
    println!("   ✓ Exported tick data to: {tick_file}");
    
    // Show sample of the tick data
    println!("   Sample data (first 3 ticks):");
    for (i, tick) in tick_data.iter().take(3).enumerate() {
        println!("     {}. {}", i + 1, format_tick(tick));
    }
    
    // Example 3: Direct export using generator methods
    println!("\n3. Using direct generator export methods...");
    
    // Reset generator for consistent results
    let config_direct = ConfigBuilder::new()
        .starting_price_f64(100.0)
        .volatility_f64(0.02)
        .seed(123)
        .build()?;
    
    let mut generator_direct = MarketDataGenerator::with_config(config_direct)?;
    
    // Generate and export in one step
    let direct_ohlc_file = "direct_export_ohlc.csv";
    generator_direct.generate_to_csv_ohlc(25, direct_ohlc_file)?;
    println!("   ✓ Generated and exported 25 OHLC candles to: {direct_ohlc_file}");
    
    let direct_tick_file = "direct_export_ticks.csv";
    generator_direct.generate_to_csv_ticks(25, direct_tick_file)?;
    println!("   ✓ Generated and exported 25 ticks to: {direct_tick_file}");
    
    // Example 4: Large dataset with streaming
    println!("\n4. Streaming large dataset export...");
    
    let large_ohlc_file = "large_dataset_ohlc.csv";
    let count = generator_direct.stream_generate_to_csv_ohlc(1000, large_ohlc_file)?;
    println!("   ✓ Streamed {count} OHLC candles to: {large_ohlc_file}");
    
    // Example 5: Custom CSV options
    println!("\n5. Custom CSV export options...");
    
    use market_data_source::export::csv::{CsvExporter, QuoteStyle};
    use market_data_source::export::DataExporter;
    
    // Create data for custom export
    let custom_data = generator.generate_series(10);
    
    // Export with semicolon delimiter
    let semicolon_exporter = CsvExporter::new()
        .delimiter(b';')
        .quote_style(QuoteStyle::Always);
    
    let semicolon_file = "custom_semicolon.csv";
    semicolon_exporter.export_ohlc(&custom_data, semicolon_file)?;
    println!("   ✓ Exported with semicolon delimiter to: {semicolon_file}");
    
    // Export without headers
    let no_headers_exporter = CsvExporter::new()
        .include_headers(false);
    
    let no_headers_file = "no_headers.csv";
    no_headers_exporter.export_ohlc(&custom_data, no_headers_file)?;
    println!("   ✓ Exported without headers to: {no_headers_file}");
    
    println!("\n✅ CSV export examples completed successfully!");
    println!("\nGenerated files:");
    println!("  - {ohlc_file}");
    println!("  - {tick_file}");
    println!("  - {direct_ohlc_file}");
    println!("  - {direct_tick_file}");
    println!("  - {large_ohlc_file}");
    println!("  - {semicolon_file}");
    println!("  - {no_headers_file}");
    
    println!("\nYou can open these files in Excel, Google Sheets, or any CSV viewer.");
    println!("Or analyze them with Python pandas: pd.read_csv('{ohlc_file}').");
    
    Ok(())
}

// Helper function to format OHLC data for display
fn format_ohlc(ohlc: &market_data_source::OHLC) -> String {
    format!(
        "O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}",
        ohlc.open, ohlc.high, ohlc.low, ohlc.close, ohlc.volume
    )
}

// Helper function to format tick data for display
fn format_tick(tick: &market_data_source::Tick) -> String {
    let bid_str = tick.bid.map(|b| format!("{b:.2}")).unwrap_or_else(|| "N/A".to_string());
    let ask_str = tick.ask.map(|a| format!("{a:.2}")).unwrap_or_else(|| "N/A".to_string());
    format!(
        "Price:{:.2} Vol:{} Bid:{} Ask:{}",
        tick.price, tick.volume, bid_str, ask_str
    )
}
