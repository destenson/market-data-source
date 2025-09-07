#![allow(unused)]
//! Basic example of using the market-data-source library

use market_data_source::MarketDataGenerator;

fn main() {
    println!("Market Data Source - Basic Example");
    println!("==================================");
    
    // Create a generator with default configuration
    let mut generator = MarketDataGenerator::new();
    
    // Generate some OHLC candles
    let candles = generator.generate_series(10);
    
    println!("\nGenerated {} candles:", candles.len());
    for (i, candle) in candles.iter().enumerate() {
        println!("Candle {}: {:?}", i + 1, candle);
    }
}
