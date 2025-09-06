//! # Market Data Source
//!
//! A Rust library for generating realistic synthetic market data with unparalleled configurability.
//!
//! ## Features
//!
//! - Configurable price generation with trend and volatility control
//! - Multiple time intervals (1min, 5min, 1hr, daily, etc.)
//! - OHLC (Open, High, Low, Close) candle generation
//! - Deterministic generation with seed support
//! - Statistical validation of generated data
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use market_data_source::{MarketDataGenerator, GeneratorConfig};
//!
//! // Create a generator with default configuration
//! let mut generator = MarketDataGenerator::new();
//!
//! // Generate a series of OHLC candles
//! let candles = generator.generate_series(100);
//!
//! for candle in &candles[..5] {
//!     println!("{:?}", candle);
//! }
//! ```

pub mod types;
pub mod config;
pub mod generator;
pub mod algorithms;

// Re-export main types for convenience
pub use types::{OHLC, Tick, TimeInterval, Volume};
pub use config::{GeneratorConfig, ConfigBuilder, TrendDirection};
pub use generator::MarketDataGenerator;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Verify that main types are accessible
        let _ = VERSION;
    }
}