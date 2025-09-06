# Market Data Source

Market Data Source is a rust library for fetching and processing market data from various financial APIs. It provides a unified interface to access real-time and historical market data, making it easier for developers to integrate market data into their applications.

One of its key features is its extensible architecture, allowing users to easily add support for new data sources as needed.

Another key feature is realistic data generation, enabling users to simulate market conditions and test their trading strategies without risking real capital. It includes various algorithms to generate synthetic market data that mimics real-world behavior. It also supports customization of data generation parameters to fit specific testing scenarios.

## Features

- Configurable synthetic market data generation
- Support for OHLC candles and tick data
- Random walk with drift algorithm
- Multiple export formats:
  - CSV export for data analysis
  - JSON and JSON Lines export
  - CouchDB integration for NoSQL storage
- Deterministic generation with seed support
- Streaming generation for large datasets
- Extensible architecture for adding new algorithms

## Getting Started

To use Market Data Source in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
market-data-source = { version = "0.1.0", features = ["csv_export", "json_export", "couchdb", "dotenvy", "serde"] }
```

### Environment Variables

Market Data Source supports configuration through environment variables. Copy `.env.example` to `.env` and configure your settings:

```bash
cp .env.example .env
# Edit .env with your actual values
```

Key environment variables:
- `COUCHDB_URL`: CouchDB server URL (default: http://localhost:5984)
- `COUCHDB_USERNAME` / `COUCHDB_PASSWORD`: Optional authentication
- `COUCHDB_DATABASE`: Database name (default: market_data)
- `EXPORT_BATCH_SIZE`: Batch size for bulk operations (default: 1000)
- API keys for future data providers (Alpha Vantage, Polygon, Finnhub, etc.)

See `.env.example` for the complete list of available environment variables.

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

### Data Export

Market Data Source supports multiple export formats for different use cases:

#### CSV Export

Export generated data to CSV files for analysis in Excel, pandas, or other tools:

```rust
use market_data_source::{MarketDataGenerator, GeneratorConfig, export::CsvExporter, export::DataExporter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GeneratorConfig::default()
        .with_initial_price(100.0)
        .with_volatility(0.02);
    
    let mut generator = MarketDataGenerator::new(config);
    let ohlc_data = generator.generate_ohlc(100);
    
    // Export OHLC data to CSV
    let csv_exporter = CsvExporter::default();
    csv_exporter.export_ohlc(&ohlc_data, "market_data.csv")?;
    
    Ok(())
}
```

#### JSON Export

Export data in JSON or JSON Lines format:

```rust
use market_data_source::{export::JsonExporter, export::JsonOptions, export::DataExporter};

// Standard JSON format
let json_exporter = JsonExporter::default();
json_exporter.export_ohlc(&ohlc_data, "market_data.json")?;

// JSON Lines format (one JSON object per line)
let jsonl_exporter = JsonExporter::with_options(JsonOptions::json_lines());
jsonl_exporter.export_ohlc(&ohlc_data, "market_data.jsonl")?;
```

#### CouchDB Export

Export data directly to CouchDB for NoSQL storage and replication:

```rust
use market_data_source::{export::CouchDbExporter, export::DataExporter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = MarketDataGenerator::default();
    let ohlc_data = generator.generate_ohlc(100);
    
    // Method 1: Create CouchDB exporter with explicit configuration
    let couchdb_exporter = CouchDbExporter::new("http://localhost:5984", "market_data")
        .with_auth("admin", "password")  // Optional authentication
        .with_batch_size(500);           // Configure batch size for bulk operations
    
    // Method 2: Create CouchDB exporter from environment variables
    // Reads COUCHDB_URL, COUCHDB_USERNAME, COUCHDB_PASSWORD, etc. from .env
    #[cfg(feature = "dotenvy")]
    let couchdb_exporter = CouchDbExporter::from_env();
    
    // Export to CouchDB
    couchdb_exporter.export_ohlc(&ohlc_data, "")?;
    
    // The data is now stored in CouchDB with views for querying:
    // - by_timestamp: Query data by timestamp
    // - by_symbol_and_timestamp: Query by symbol and timestamp
    // - ohlc_by_date_range: Query OHLC data within a date range
    // - ticks_by_date_range: Query tick data within a date range
    
    Ok(())
}
```

For convenience, you can also use the helper functions:

```rust
use market_data_source::export::{to_csv_ohlc, to_json_ohlc, to_couchdb_ohlc};

// Quick export to different formats
to_csv_ohlc(&ohlc_data, "data.csv")?;
to_json_ohlc(&ohlc_data, "data.json")?;
to_couchdb_ohlc(&ohlc_data, "http://localhost:5984", "market_db")?;

// Or use environment variables for CouchDB
#[cfg(all(feature = "couchdb", feature = "dotenvy"))]
use market_data_source::export::{to_couchdb_ohlc_env, to_couchdb_ticks_env};

#[cfg(all(feature = "couchdb", feature = "dotenvy"))]
to_couchdb_ohlc_env(&ohlc_data)?;  // Uses COUCHDB_URL, COUCHDB_DATABASE from .env
```

## Current Status

✅ **v0.1.0 Foundation Complete**
- Library structure implemented
- Core data types (OHLC, Tick, Volume)
- Market data generator with configurable parameters
- Random walk with drift algorithm
- Builder pattern for configuration
- Multiple export formats:
  - CSV export functionality with streaming support
  - JSON and JSON Lines export
  - CouchDB integration with bulk operations and views
- Serde serialization support
- 45+ tests passing (unit tests + integration tests)
- Working examples

🚧 **In Development**
- Additional generation algorithms
- More sophisticated market patterns
- API emulation features

## Contributing

Contributions are welcome! If you'd like to contribute to Market Data Source, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


