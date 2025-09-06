# Market Data Source

Market Data Source is a rust library for fetching and processing market data from various financial APIs. It provides a unified interface to access real-time and historical market data, making it easier for developers to integrate market data into their applications.

One of its key features is its extensible architecture, allowing users to easily add support for new data sources as needed.

Another key feature is realistic data generation, enabling users to simulate market conditions and test their trading strategies without risking real capital. It includes various algorithms to generate synthetic market data that mimics real-world behavior. It also supports customization of data generation parameters to fit specific testing scenarios.

## Features

- Unified API for multiple financial data sources
- Support for real-time and historical data
- Easy integration with existing Rust applications
- Extensible architecture for adding new data sources

## Getting Started

To use Market Data Source in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
market-data-source = "0.1.0"
```

## Usage

Here's a simple example of how to use Market Data Source to generate synthetic market data:

```rust
use market_data_source::{MarketDataGenerator, GeneratorConfig};

fn main() {
    // Create a generator with default configuration
    let mut generator = MarketDataGenerator::new();
    
    // Generate 10 OHLC candles
    let candles = generator.generate_series(10);
    
    for candle in &candles[..3] {
        println!("{:?}", candle);
    }
    
    // Create a generator with custom configuration
    let config = GeneratorConfig::builder()
        .starting_price(100.0)
        .volatility(0.02)  // 2% volatility
        .trend_strength(0.001)  // Slight upward trend
        .seed(42)  // For reproducible results
        .build();
    
    let mut custom_generator = MarketDataGenerator::with_config(config);
    let custom_candles = custom_generator.generate_series(5);
}
```

## Current Status

✅ **v0.1.0 Foundation Complete**
- Library structure implemented
- Core data types (OHLC, Tick, Volume)
- Market data generator with configurable parameters
- Random walk with drift algorithm
- Builder pattern for configuration
- 27 unit tests passing
- Working examples

🚧 **In Development**
- Additional generation algorithms
- More sophisticated market patterns
- API emulation features

## Contributing

Contributions are welcome! If you'd like to contribute to Market Data Source, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


