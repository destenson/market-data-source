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

## Automated Releases

Market Data Source uses GitHub Actions with trusted publishing (OIDC) for secure, automated releases to both crates.io and PyPI without storing API tokens.

### Release Process

To create a new release:

1. Update version in `Cargo.toml` and `pyproject.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit and push changes
4. Create and push a version tag:
   ```bash
   git tag v0.3.1
   git push origin v0.3.1
   ```

The automated workflow will:
- Validate version consistency
- Run full test suite across all platforms
- Build and publish to crates.io
- Build Python wheels for multiple platforms
- Publish to PyPI
- Create GitHub release with artifacts

### Testing Publication Setup

Test the publishing configuration without releasing:

```bash
# Via GitHub Actions UI:
# Actions → Test Publishing Setup → Run workflow
```

For more details, see [.github/PUBLISHING.md](.github/PUBLISHING.md).

## Release Process

### Automated Release Pipeline

Market Data Source uses a fully automated release pipeline that publishes to both crates.io and PyPI. The release process is triggered by git tags and ensures consistency across both package ecosystems.

#### Quick Release

For maintainers, the simplest way to create a release:

```bash
# Prepare a patch release (e.g., 0.3.0 -> 0.3.1)
python scripts/prepare-release.py --bump patch

# Or prepare a specific version
python scripts/prepare-release.py --version 0.4.0

# Review and update CHANGELOG.md with actual changes
$EDITOR CHANGELOG.md

# Commit and push the tag
git add -A
git commit -m "chore: prepare release v0.3.1"
git push origin main
git push origin v0.3.1  # This triggers the automated release
```

#### Release Scripts

The project includes helper scripts for managing releases:

1. **Version Synchronization** (`scripts/sync-version.py`)
   - Ensures version consistency between Cargo.toml and pyproject.toml
   - Uses Cargo.toml as the single source of truth
   ```bash
   # Check version consistency
   python scripts/sync-version.py --check
   
   # Sync versions (Cargo.toml -> pyproject.toml)
   python scripts/sync-version.py
   
   # Set a specific version
   python scripts/sync-version.py --set-version 0.3.1
   ```

2. **Release Preparation** (`scripts/prepare-release.py`)
   - Automates the entire release preparation process
   - Runs quality checks, updates versions, and creates git tags
   ```bash
   # Dry run to see what would happen
   python scripts/prepare-release.py --bump patch --dry-run
   
   # Prepare a minor release
   python scripts/prepare-release.py --bump minor
   
   # Prepare a specific version
   python scripts/prepare-release.py --version 0.4.0
   ```

#### Release Workflow

The automated release pipeline (`release-automation.yml`) performs these steps:

1. **Pre-flight Validation**
   - Version format validation
   - Version consistency check across all files
   - Changelog extraction

2. **Quality Gates**
   - Runs on Ubuntu, Windows, and macOS
   - Format checking with rustfmt
   - Linting with clippy
   - Full test suite execution
   - Documentation building

3. **Build Artifacts**
   - Rust binaries for multiple platforms
   - Python wheels for multiple Python versions

4. **Publication**
   - Publishes to crates.io using OIDC trusted publishing
   - Publishes to PyPI using OIDC trusted publishing
   - Creates GitHub release with all assets

5. **Post-Release**
   - Automatically bumps version for next development cycle
   - Creates PR for version bump
   - Generates release summary

#### Manual Release Process

If you need to release manually:

```bash
# 1. Ensure versions are synchronized
python scripts/sync-version.py --check

# 2. Run all quality checks
cargo fmt --all -- --check
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --no-deps --all-features

# 3. Build and test Python package
cd market-data-source-python
maturin build --release
pip install ../target/wheels/*.whl
python -c "import market_data_source; print(market_data_source.__version__)"
cd ..

# 4. Create and push tag
git tag -a v0.3.1 -m "Release version 0.3.1"
git push origin v0.3.1

# The automated workflow will handle the rest
```

#### Version Management

- Versions follow semantic versioning (MAJOR.MINOR.PATCH)
- Cargo.toml is the single source of truth for versions
- The CI automatically checks version consistency on every push
- Pre-release versions are supported (e.g., 0.3.1-alpha.1)

#### Troubleshooting Releases

If a release fails:

1. Check the GitHub Actions logs for specific errors
2. Ensure OIDC trusted publishing is configured for both crates.io and PyPI
3. Verify version consistency: `python scripts/sync-version.py --check`
4. Run quality checks locally: `cargo test --all-features`
5. Try a dry run first: `python scripts/prepare-release.py --dry-run`

For rollback procedures, see the release workflow documentation in `.github/workflows/`.

## Contributing

Contributions are welcome! If you'd like to contribute to Market Data Source, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


