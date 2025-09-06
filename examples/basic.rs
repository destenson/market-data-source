use market_data_source::{MarketDataGenerator, GeneratorConfig};

fn main() {
    println!("Market Data Generator - Basic Example");
    println!("======================================");
    
    // Create a generator with default configuration
    let mut generator = MarketDataGenerator::new();
    
    // Generate 10 OHLC candles
    let candles = generator.generate_series(10);
    
    // Display the generated data
    for (i, candle) in candles.iter().enumerate() {
        println!("Candle {}: {:?}", i + 1, candle);
    }
    
    println!("\n--- Custom Configuration Example ---");
    
    // Create a generator with custom configuration
    let config = GeneratorConfig::builder()
        .starting_price(100.0)
        .volatility(0.02)
        .trend_strength(0.001)
        .build();
    
    let mut custom_generator = MarketDataGenerator::with_config(config);
    let custom_candles = custom_generator.generate_series(5);
    
    for (i, candle) in custom_candles.iter().enumerate() {
        println!("Custom Candle {}: {:?}", i + 1, candle);
    }
}