//! # Market Data Source
//!
//! A Rust library for generating realistic synthetic market data with unparalleled configurability.
//! 
//! ## Features
//! 
//! - Configurable market data generation with statistical controls
//! - Support for OHLC candles and tick data
//! - Random walk with drift algorithm
//! - Builder pattern for easy configuration
//! - Deterministic generation with seed support
//!
//! ## Quick Start
//!
//! ```no_run
//! use market_data_source::{MarketDataGenerator, GeneratorConfig};
//!
//! // Create a generator with default config
//! let mut generator = MarketDataGenerator::new();
//! 
//! // Generate a series of OHLC candles
//! let candles = generator.generate_series(100);
//! 
//! for candle in &candles[..5] {
//!     println!("{:?}", candle);
//! }
//! ```

// Public modules
pub mod config;
pub mod generator;
pub mod types;

// Export module (conditional on feature flags)
#[cfg(any(feature = "csv_export", feature = "json_export", feature = "couchdb"))]
pub mod export;

// Re-export main types for convenience
pub use config::{GeneratorConfig, ConfigBuilder, TrendDirection};
pub use generator::MarketDataGenerator;
pub use types::{OHLC, Tick, TimeInterval, Volume};

// Re-export export functionality when feature is enabled
#[cfg(feature = "csv_export")]
pub use export::{to_csv_ohlc, to_csv_ticks};

// Module for algorithms (internal for now)
mod algorithms;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_imports_work() {
        // Basic smoke test that library structure works
        let _config = GeneratorConfig::default();
    }
}