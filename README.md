# Market Data Source

[![Crates.io](https://img.shields.io/crates/v/market-data-source.svg)](https://crates.io/crates/market-data-source)
[![PyPI](https://img.shields.io/pypi/v/market-data-source.svg)](https://pypi.org/project/market-data-source/)
[![Documentation](https://docs.rs/market-data-source/badge.svg)](https://docs.rs/market-data-source)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://github.com/deste128/market-data-source/actions/workflows/test.yml/badge.svg)](https://github.com/deste128/market-data-source/actions/workflows/test.yml)
[![Python Tests](https://github.com/deste128/market-data-source/actions/workflows/python-test.yml/badge.svg)](https://github.com/deste128/market-data-source/actions/workflows/python-test.yml)
[![Code Quality](https://github.com/deste128/market-data-source/actions/workflows/quality.yml/badge.svg)](https://github.com/deste128/market-data-source/actions/workflows/quality.yml)

**Financial precision with Decimal types** | **Python & Rust dual ecosystem support** | **Extensible architecture**

Market Data Source is a Rust library for generating realistic synthetic market data with Python bindings. It provides a unified interface to generate real-time and historical market data, making it easier for developers to integrate market data into their applications for backtesting, research, and development.

## Why Market Data Source?

- **No API limits** - Generate unlimited data without rate limiting
- **No costs** - Completely free and open source
- **Reproducible** - Deterministic generation with seed support
- **Native bindings** - Use from both Rust and Python
- **Flexible** - Export to CSV, JSON, PNG charts, or stream to CouchDB

Key features:
- **Rust implementation** with Python bindings via PyO3
- **Realistic data generation** using advanced algorithms to simulate market conditions
- **Multiple export formats** including CSV, JSON, CouchDB, and PNG charts
- **Extensible architecture** allowing users to easily add custom algorithms

## Features

- Configurable synthetic market data generation
- Support for OHLC candles and tick data
- Random walk with drift algorithm
- Multiple export formats:
  - CSV export for data analysis
  - JSON and JSON Lines export
  - CouchDB integration for NoSQL storage
  - PNG chart generation for data visualization
- Deterministic generation with seed support
- Streaming generation for large datasets
- Extensible architecture for adding new algorithms

## Installation

### Quick Install

#### Python (pip)
```bash
pip install market-data-source
```

#### Rust (Cargo)
```toml
[dependencies]
market-data-source = "0.3.0"
```

### Detailed Installation

#### Python Installation

**Option 1: Install from PyPI (Recommended)**
```bash
# Using pip
pip install market-data-source

# Using uv (faster)
uv pip install market-data-source
```

**Option 2: Build from source**
```bash
# Clone the repository
git clone https://github.com/yourusername/market-data-source.git
cd market-data-source

# Install with maturin
pip install maturin
maturin develop --release
```

#### Rust Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
market-data-source = "0.3.0"

# Or with specific features
market-data-source = { 
    version = "0.3.0", 
    features = ["csv_export", "json_export", "png_export"] 
}
```

Available features:
- `csv_export` - CSV file export support
- `json_export` - JSON/JSONL export support  
- `png_export` - PNG chart generation
- `couchdb` - CouchDB database export
- `dotenvy` - Environment variable configuration
- `serde` - Serialization support

## Quick Start

### Python Example

```python
import market_data_source as mds

# Create generator with configuration
generator = mds.MarketDataGenerator(
    initial_price=100.0,
    volatility=0.02,
    seed=42
)

# Generate 100 OHLC candles
data = generator.generate_series(100)

# Export to various formats
generator.to_csv("data.csv", count=1000)
generator.to_json("data.json", count=1000)
generator.to_png("chart.png", count=500)
```

### Rust Example

```rust
use market_data_source::{MarketDataGenerator, ConfigBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create generator with custom config
    let config = ConfigBuilder::new()
        .initial_price_f64(100.0)
        .volatility_f64(0.02)
        .seed(42)
        .build();
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    
    // Generate OHLC data
    let candles = generator.generate_series(100);
    
    // Export to CSV
    #[cfg(feature = "csv_export")]
    {
        use market_data_source::export::to_csv_ohlc;
        to_csv_ohlc(&candles, "output.csv")?;
    }
    
    Ok(())
}
```

See [examples/](examples/) directory for more comprehensive examples.

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

### Python Usage

Market Data Source provides a Pythonic API for generating market data:

```python
import market_data_source as mds
import pandas as pd

# Create generator with configuration
generator = mds.MarketDataGenerator(
    initial_price=100.0,
    volatility=0.02,      # 2% volatility
    trend=0.0001,         # Slight upward trend
    volume_base=1000000,  # Base volume
    interval="1m",        # 1-minute bars
    seed=42              # Reproducible results
)

# Generate OHLC data
ohlc_data = generator.generate_series(100)

# Convert to pandas DataFrame
df = pd.DataFrame(ohlc_data)
df['datetime'] = pd.to_datetime(df['timestamp'], unit='s')

# Generate tick data
ticks = generator.generate_ticks(1000)

# Use preset configurations
volatile_gen = mds.volatile_config()
stable_gen = mds.stable_config()
bull_gen = mds.bull_market_config()
bear_gen = mds.bear_market_config()

# Export data
generator.to_csv("data.csv", count=1000)
generator.to_json("data.json", count=1000)
generator.to_png("chart.png", count=500, width=1200, height=800)
```

See `examples/python/` for more complete examples including pandas integration.

### Rust Usage

Here's a simple example of how to use Market Data Source in Rust:

```rust
use market_data_source::{MarketDataGenerator, ConfigBuilder, TrendDirection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a generator with default configuration
    let mut generator = MarketDataGenerator::new();
    
    // Generate 10 OHLC candles
    let candles = generator.generate_series(10);
    
    for candle in &candles[..3] {
        println!("{:?}", candle);
    }
    
    // Create a generator with custom configuration
    let config = ConfigBuilder::new()
        .initial_price_f64(100.0)
        .volatility_f64(0.02)  // 2% volatility
        .trend_f64(0.001)  // Slight upward trend
        .seed(42)  // For reproducible results
        .build();
    
    let mut custom_generator = MarketDataGenerator::with_config(config)?;
    let custom_candles = custom_generator.generate_series(5);
    
    Ok(())
}
```

### Data Export

Market Data Source supports multiple export formats for different use cases:

#### CSV Export

Export generated data to CSV files for analysis in Excel, pandas, or other tools:

```rust
use market_data_source::{MarketDataGenerator, ConfigBuilder, TrendDirection};
use market_data_source::export::{to_csv_ohlc, to_csv_ticks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
        .starting_price(100.0)
        .volatility(0.02)
        .trend(TrendDirection::Bullish, 0.001)
        .build()?;
    
    let mut generator = MarketDataGenerator::with_config(config)?;
    let ohlc_data = generator.generate_series(100);
    
    // Export OHLC data to CSV using convenience function
    to_csv_ohlc(&ohlc_data, "market_data.csv")?;
    
    // Or use the exporter directly for custom options
    use market_data_source::export::csv::CsvExporter;
    use market_data_source::export::DataExporter;
    
    let csv_exporter = CsvExporter::new()
        .delimiter(b';')  // Use semicolon delimiter
        .include_headers(true);
    csv_exporter.export_ohlc(&ohlc_data, "market_data_custom.csv")?;
    
    Ok(())
}
```

#### JSON Export

Export data in JSON or JSON Lines format:

```rust
use market_data_source::export::{to_json_ohlc, to_jsonl_ohlc};

// Standard JSON format using convenience function
to_json_ohlc(&ohlc_data, "market_data.json")?;

// JSON Lines format (one JSON object per line)
to_jsonl_ohlc(&ohlc_data, "market_data.jsonl")?;

// Or use the exporter directly for custom options
use market_data_source::export::json::{JsonExporter, JsonOptions};
use market_data_source::export::DataExporter;

// Pretty-printed JSON
let json_exporter = JsonExporter::with_options(JsonOptions::pretty());
json_exporter.export_ohlc(&ohlc_data, "pretty_data.json")?;

// JSON Lines format
let jsonl_exporter = JsonExporter::with_options(JsonOptions::json_lines());
jsonl_exporter.export_ohlc(&ohlc_data, "streaming_data.jsonl")?;
```

#### CouchDB Export

Export data directly to CouchDB for NoSQL storage and replication:

```rust
use market_data_source::export::{to_couchdb_ohlc, to_couchdb_ticks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = MarketDataGenerator::new();
    let ohlc_data = generator.generate_series(100);
    
    // Export using convenience functions
    to_couchdb_ohlc(&ohlc_data, "http://localhost:5984", "market_data")?;
    
    // Or use environment variables (requires dotenvy feature)
    #[cfg(all(feature = "couchdb", feature = "dotenvy"))]
    {
        use market_data_source::export::{to_couchdb_ohlc_env, to_couchdb_ticks_env};
        to_couchdb_ohlc_env(&ohlc_data)?;
    }
    
    // For custom options, use the exporter directly
    use market_data_source::export::couchdb::{CouchDbExporter, CouchDbOptions};
    use market_data_source::export::DataExporter;
    
    let options = CouchDbOptions::new()
        .timeout_seconds(30)
        .batch_size(100)
        .auto_create_database(true);
        
    let couchdb_exporter = CouchDbExporter::new_with_options(
        "http://localhost:5984", 
        "market_data", 
        options
    );
    couchdb_exporter.export_ohlc(&ohlc_data, "")?;
    
    Ok(())
}
```

### Running Examples

The library includes comprehensive examples for all export formats:

```bash
# CSV export example
cargo run --example export_csv --features csv_export

# JSON export example  
cargo run --example export_json --features json_export

# CouchDB export example (requires CouchDB running)
cargo run --example export_couchdb --features couchdb

# PNG chart export example
cargo run --example export_charts --features png_export

# All export formats example
cargo run --example export_all --all-features
```

## Current Status

✅ **v0.1.0 Foundation Complete**
- Library structure implemented
- Core data types (OHLC, Tick, Volume)
- Market data generator with configurable parameters
- Random walk with drift algorithm
- Builder pattern for configuration
- Comprehensive export infrastructure:
  - CSV export with streaming support and custom options
  - JSON and JSON Lines export with pretty printing
  - CouchDB integration with bulk operations
  - PNG chart generation with candlestick charts, line charts, volume bars, and moving averages
- Proper error handling with structured error types
- Serde serialization support
- 54+ tests passing (unit tests + integration tests + comprehensive export tests)
- Complete example suite demonstrating all export formats

🚧 **In Development**
- Additional generation algorithms
- More sophisticated market patterns
- API emulation features

## Contributing

Contributions are welcome! If you'd like to contribute to Market Data Source, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


