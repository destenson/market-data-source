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

// Environment configuration module (conditional on feature flag)
#[cfg(feature = "dotenvy")]
pub mod env;

// Export module (conditional on feature flags)
#[cfg(any(
    feature = "csv_export",
    feature = "json_export",
    feature = "couchdb",
    feature = "png_export"
))]
pub mod export;

// Re-export main types for convenience
pub use config::{ConfigBuilder, GeneratorConfig, TrendDirection};
pub use generator::MarketDataGenerator;
pub use types::{Tick, TimeInterval, Volume, OHLC};

// Re-export export functionality when feature is enabled
#[cfg(feature = "csv_export")]
pub use export::{to_csv_ohlc, to_csv_ticks};

// Module for algorithms (internal implementation details)
mod algorithms;

// Regime detection module (conditional on feature flag)
#[cfg(feature = "regimes")]
pub mod regimes;

// Re-export regime types when feature is enabled
#[cfg(feature = "regimes")]
pub use regimes::{MarketRegime, RegimeController, RegimeSchedule, RegimeSegment, ScheduleInfo};

// Re-export generator regime types when feature is enabled
#[cfg(feature = "regimes")]
pub use generator::{ControlledRegimeOHLC, RegimeOHLC};

// Server module (conditional on feature flag)
#[cfg(feature = "api-server")]
pub mod server;
