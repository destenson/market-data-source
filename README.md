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

Here's a simple example of how to use Market Data Source to fetch market data:

```rust
use market_data_source::MarketData;

fn main() {
    let market_data = MarketData::new();

    // Fetch real-time market data
    let real_time_data = market_data.fetch_real_time_data("AAPL");
    println!("Real-time data: {:?}", real_time_data);

    // Fetch historical market data
    let historical_data = market_data.fetch_historical_data("AAPL", "2022-01-01", "2022-01-31");
    println!("Historical data: {:?}", historical_data);
}
```

## Contributing

Contributions are welcome! If you'd like to contribute to Market Data Source, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


