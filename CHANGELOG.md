# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-01-07

### Added
- Enhanced feature set with synthetic and live data capabilities
- Server infrastructure with REST/WebSocket API using Axum framework
- Comprehensive Python bindings with PyO3 integration
- Environment variable configuration support via dotenvy
- Multiple export formats:
  - CSV export with streaming support and custom options
  - JSON and JSON Lines export with pretty printing
  - CouchDB integration with bulk operations
  - PNG chart generation with candlestick charts, line charts, volume bars, and moving averages
- Configuration presets (volatile, stable, bull market, bear market)
- Comprehensive test suite with 64 unit tests and 11 integration tests

### Changed
- Migrated from deprecated PyO3 traits to modern implementations
- Updated error handling to use I/O error variants throughout
- Improved API with builder pattern for configuration
- Enhanced documentation with comprehensive examples
- Optimized memory management for complex feature builds

### Fixed
- Deprecated candle generation replaced with updated OHLC generation
- Compilation errors resolved across all feature combinations
- Error handling improvements in CSV and PNG export functions
- Memory optimization for parallel builds (requires `-j 1` flag)

## [0.2.0] - 2024-12-15

### Added
- Python bindings via PyO3 for seamless Python integration
- Financial precision with rust_decimal::Decimal types for all monetary values
- Export infrastructure foundation (19 PRPs completed):
  - DataExporter trait for extensible export capabilities
  - CSV export module with configurable options
  - JSON export module supporting both standard and JSON Lines format
  - CouchDB export module with bulk operations support
  - PNG chart export module with multiple visualization types
- Comprehensive integration tests for all export formats
- Example suite demonstrating all export capabilities
- ConfigBuilder with fluent API for generator configuration
- Tick data generation support
- Volume generation with realistic patterns

### Changed
- Refactored core data types to use Decimal for financial precision
- Improved random walk algorithm with drift and volatility parameters
- Enhanced error handling with structured error types
- Modularized export functionality with feature flags

### Fixed
- Floating-point precision issues in financial calculations
- Test helper module issues resolved

## [0.1.0] - 2024-11-01

### Added
- Initial release with core market data generation capabilities
- Library structure with modular architecture
- Core data types:
  - OHLC (Open, High, Low, Close) candle representation
  - Tick data structure for individual trades
  - Volume data with buy/sell pressure metrics
- MarketDataGenerator with configurable parameters:
  - Initial price setting
  - Volatility control
  - Trend direction and strength
  - Time interval configuration
- Random walk with drift algorithm for realistic price movements
- Timestamp generation with proper market hours simulation
- Deterministic generation with seed support for reproducible results
- Basic example demonstrating core functionality
- Unit test suite with comprehensive coverage

### Security
- No security vulnerabilities in initial release

## [Unreleased]

### Planned
- Additional generation algorithms (GARCH, mean reversion, jump diffusion)
- Real data source integration (Yahoo Finance, Alpha Vantage)
- Market regime changes (dynamic bull/bear/sideways transitions)
- Live parameter updates (runtime configuration without restart)
- WebSocket streaming improvements
- Level 2 order book generation
- Options pricing models
- Multi-asset correlation support

---

*This project is under active development. For the latest updates, see the [GitHub repository](https://github.com/yourusername/market-data-source).*

[0.3.0]: https://github.com/yourusername/market-data-source/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/yourusername/market-data-source/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yourusername/market-data-source/releases/tag/v0.1.0