#![allow(unused)]
//! PNG Chart Export Example
//!
//! This example demonstrates how to export market data as PNG charts.
//! It covers candlestick charts for OHLC data and line charts for tick data,
//! with various styling and visualization options.

#[cfg(feature = "png_export")]
use market_data_source::{
    MarketDataGenerator, ConfigBuilder, TrendDirection,
    export::{to_png_ohlc, to_png_ticks, ChartBuilder}
};

#[cfg(feature = "png_export")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Market Data Source - PNG Chart Export Example");
    println!("=============================================");
    
    // Create a generator with trending data for interesting charts
    let config = ConfigBuilder::new()
        .starting_price_f64(120.0)
        .volatility_f64(0.025)
        .trend_f64(TrendDirection::Bullish, 0.002)
        .seed(321)  // For reproducible results
        .build()?;
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    
    // Example 1: Basic candlestick chart for OHLC data
    println!("\n1. Creating basic candlestick chart...");
    
    // Generate OHLC data
    let ohlc_data = generator.generate_series(50);
    println!("   Generated {} OHLC candles", ohlc_data.len());
    
    // Export as PNG chart using convenience function
    let basic_chart_file = "basic_candlestick_chart.png";
    to_png_ohlc(&ohlc_data, basic_chart_file)?;
    println!("   ✓ Created basic candlestick chart: {basic_chart_file}");
    
    // Example 2: Basic line chart for tick data
    println!("\n2. Creating basic line chart for tick data...");
    
    // Generate tick data
    let tick_data = generator.generate_ticks(100);
    println!("   Generated {} ticks", tick_data.len());
    
    // Export as PNG line chart
    let basic_line_file = "basic_line_chart.png";
    to_png_ticks(&tick_data, basic_line_file)?;
    println!("   ✓ Created basic line chart: {basic_line_file}");
    
    // Example 3: Custom styled candlestick chart
    println!("\n3. Creating custom styled candlestick chart...");
    
    use market_data_source::export::{to_png_ohlc_with_builder, ChartExporter};
    // Create custom chart builder with styling
    let custom_builder = ChartBuilder::new()
        .title("Market Data - Custom Styled Chart")
        .dimensions(1200, 800)
        .background_color((240, 248, 255))  // Alice blue background
        .grid_color((200, 200, 200))
        .show_volume(true)
        .candlestick_colors((0, 128, 0), (255, 0, 0));  // Green up, red down

    let custom_chart_file = "custom_styled_candlestick.png";
    to_png_ohlc_with_builder(&ohlc_data, custom_chart_file, custom_builder)?;
    println!("   ✓ Created custom styled chart: {custom_chart_file}");
    
    // Example 4: Chart with moving averages
    println!("\n4. Creating chart with moving averages...");
    
    // Generate more data for better moving average visualization
    let ma_data = generator.generate_series(100);
    
    let ma_builder = ChartBuilder::new()
        .title("OHLC with Moving Averages")
        .dimensions(1000, 600)
        .show_volume(true)
        .show_sma(20)   // 20-period simple moving average
        .sma_color((255, 165, 0));  // Orange color for SMA
    
    let ma_chart_file = "chart_with_moving_average.png";
    to_png_ohlc_with_builder(&ma_data, ma_chart_file, ma_builder)?;
    println!("   ✓ Created chart with moving average: {ma_chart_file}");
    
    // Example 5: High resolution chart for presentation
    println!("\n5. Creating high resolution chart...");
    
    let hires_builder = ChartBuilder::new()
        .title("High Resolution Market Data Chart")
        .dimensions(1920, 1080)
        .background_color((255, 255, 255))  // White background
        .grid_color((230, 230, 230))
        .show_volume(true)
        .show_sma(10)
        .candlestick_colors((34, 139, 34), (220, 20, 60));  // Forest green, crimson
    
    let hires_chart_file = "high_resolution_chart.png";
    to_png_ohlc_with_builder(&ohlc_data, hires_chart_file, hires_builder)?;
    println!("   ✓ Created high resolution chart: {hires_chart_file}");
    
    // Example 6: Dark theme chart
    println!("\n6. Creating dark theme chart...");
    
    let dark_builder = ChartBuilder::new()
        .title("Dark Theme Market Chart")
        .dimensions(1000, 700)
        .background_color((30, 30, 30))     // Dark background
        .text_color((255, 255, 255))        // White text
        .grid_color((70, 70, 70))           // Dark gray grid
        .show_volume(true)
        .candlestick_colors((0, 255, 127), (255, 69, 0));  // Spring green, red orange
    
    let dark_chart_file = "dark_theme_chart.png";
    to_png_ohlc_with_builder(&ohlc_data, dark_chart_file, dark_builder)?;
    println!("   ✓ Created dark theme chart: {dark_chart_file}");
    
    // Example 7: Tick data line chart with custom styling
    println!("\n7. Creating styled tick line chart...");
    
    use market_data_source::export::to_png_ticks_with_builder;
    
    let tick_builder = ChartBuilder::new()
        .title("Tick Price Movement")
        .dimensions(1000, 500)
        .background_color((248, 248, 255))  // Ghost white
        .line_color((72, 61, 139))          // Dark slate blue
        .line_width(2);
    
    let styled_tick_file = "styled_tick_chart.png";
    to_png_ticks_with_builder(&tick_data, styled_tick_file, tick_builder)?;
    println!("   ✓ Created styled tick chart: {styled_tick_file}");
    
    // Example 8: Multiple timeframes comparison
    println!("\n8. Creating comparison charts...");
    
    // Create different datasets for comparison
    let short_term = generator.generate_series(20);
    let _medium_term = generator.generate_series(50);
    let long_term = generator.generate_series(100);
    
    // Short term chart
    let short_builder = ChartBuilder::new()
        .title("Short Term (20 periods)")
        .dimensions(800, 400)
        .show_volume(true);
    
    let short_chart_file = "short_term_chart.png";
    to_png_ohlc_with_builder(&short_term, short_chart_file, short_builder)?;
    
    // Long term chart  
    let long_builder = ChartBuilder::new()
        .title("Long Term (100 periods)")
        .dimensions(800, 400)
        .show_volume(true)
        .show_sma(20);
    
    let long_chart_file = "long_term_chart.png";
    to_png_ohlc_with_builder(&long_term, long_chart_file, long_builder)?;
    
    println!("   ✓ Created comparison charts: {short_chart_file} and {long_chart_file}");
    
    // Example 9: Chart export using ChartExporter directly
    println!("\n9. Using ChartExporter directly...");
    
    let exporter_builder = ChartBuilder::new()
        .title("Direct Exporter Usage")
        .dimensions(900, 600)
        .show_volume(true);
    
    let chart_exporter = ChartExporter::with_builder(exporter_builder);
    let direct_export_file = "direct_exporter_chart.png";
    
    chart_exporter.export_ohlc(&ohlc_data, direct_export_file)?;
    println!("   ✓ Created chart using direct exporter: {direct_export_file}");
    
    println!("\n✅ PNG chart export examples completed successfully!");
    println!("\nGenerated chart files:");
    println!("  - {basic_chart_file} (Basic candlestick)");
    println!("  - {basic_line_file} (Basic line chart)");
    println!("  - {custom_chart_file} (Custom styled)");
    println!("  - {ma_chart_file} (With moving average)");
    println!("  - {hires_chart_file} (High resolution)");
    println!("  - {dark_chart_file} (Dark theme)");
    println!("  - {styled_tick_file} (Styled tick chart)");
    println!("  - {short_chart_file} (Short term)");
    println!("  - {long_chart_file} (Long term)");
    println!("  - {direct_export_file} (Direct exporter)");
    
    println!("\nChart features demonstrated:");
    println!("  📊 Candlestick charts for OHLC data");
    println!("  📈 Line charts for tick data");
    println!("  📉 Volume bars as subplot");
    println!("  📐 Simple moving averages overlay");
    println!("  🎨 Custom colors and themes");
    println!("  📏 Flexible dimensions and resolution");
    println!("  🌙 Dark and light themes");
    
    println!("\nUsage tips:");
    println!("  - Use candlestick charts for OHLC data analysis");
    println!("  - Use line charts for continuous price movements");
    println!("  - Add volume bars to see trading activity");
    println!("  - Include moving averages for trend analysis");
    println!("  - Adjust colors and themes for presentation needs");
    println!("  - Use high resolution for printing and presentations");
    
    Ok(())
}

#[cfg(not(feature = "png_export"))]
fn main() {
    println!("PNG export feature is not enabled.");
    println!("Run with: cargo run --example export_charts --features png_export");
    std::process::exit(1);
}
