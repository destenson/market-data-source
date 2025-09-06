//! Main market data generator orchestrating the generation process.

use crate::algorithms::RandomWalkGenerator;
use crate::config::GeneratorConfig;
use crate::types::{OHLC, Tick};
use std::time::{SystemTime, UNIX_EPOCH};

/// The main market data generator that creates synthetic market data.
///
/// # Examples
///
/// ```rust,no_run
/// use market_data_source::{MarketDataGenerator, GeneratorConfig};
///
/// // Create with default configuration
/// let mut generator = MarketDataGenerator::new();
/// let candles = generator.generate_series(10);
///
/// // Create with custom configuration
/// let config = GeneratorConfig::builder()
///     .starting_price(150.0)
///     .volatility(0.03)
///     .build();
/// let mut custom_gen = MarketDataGenerator::with_config(config);
/// ```
pub struct MarketDataGenerator {
    /// The underlying random walk generator
    walker: RandomWalkGenerator,
    /// Configuration for generation
    config: GeneratorConfig,
}

impl MarketDataGenerator {
    /// Creates a new generator with default configuration.
    pub fn new() -> Self {
        let config = GeneratorConfig::default();
        let walker = RandomWalkGenerator::new(config.clone());
        
        Self { walker, config }
    }

    /// Creates a new generator with custom configuration.
    ///
    /// # Panics
    /// Panics if the configuration is invalid.
    pub fn with_config(config: GeneratorConfig) -> Self {
        config.validate().expect("Invalid configuration");
        let walker = RandomWalkGenerator::new(config.clone());
        
        Self { walker, config }
    }

    /// Generates a single OHLC candle.
    ///
    /// Each call advances the internal state and timestamp.
    pub fn generate_candle(&mut self) -> OHLC {
        self.walker.generate_candle()
    }

    /// Generates a series of OHLC candles.
    ///
    /// # Arguments
    /// * `count` - Number of candles to generate
    ///
    /// # Returns
    /// A vector of OHLC candles in chronological order.
    pub fn generate_series(&mut self, count: usize) -> Vec<OHLC> {
        let mut candles = Vec::with_capacity(count);
        
        for _ in 0..count {
            candles.push(self.generate_candle());
        }
        
        candles
    }

    /// Generates tick-level data.
    ///
    /// # Arguments
    /// * `count` - Number of ticks to generate
    ///
    /// # Returns
    /// A vector of Tick data points.
    pub fn generate_ticks(&mut self, count: usize) -> Vec<Tick> {
        let mut ticks = Vec::with_capacity(count);
        let tick_interval = 1000; // 1 second between ticks
        
        let mut timestamp = self.get_current_timestamp();
        
        for _ in 0..count {
            let price = self.walker.next_price();
            let volume = self.walker.generate_volume();
            
            ticks.push(Tick::new(price, volume, timestamp));
            timestamp += tick_interval;
        }
        
        ticks
    }

    /// Sets the random seed for reproducible generation.
    ///
    /// This will reset the generator to its initial state with the new seed.
    pub fn set_seed(&mut self, seed: u64) {
        self.config.seed = Some(seed);
        self.walker = RandomWalkGenerator::new(self.config.clone());
    }

    /// Resets the generator to its initial state.
    ///
    /// Price returns to starting price and timestamps reset.
    pub fn reset(&mut self) {
        self.walker.reset();
    }

    /// Sets the starting timestamp for generation.
    ///
    /// # Arguments
    /// * `timestamp` - Unix timestamp in milliseconds
    pub fn set_start_timestamp(&mut self, timestamp: i64) {
        self.walker.set_start_timestamp(timestamp);
    }

    /// Sets the starting timestamp to the current system time.
    pub fn use_current_time(&mut self) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;
        
        self.set_start_timestamp(timestamp);
    }

    /// Gets the current configuration.
    pub fn config(&self) -> &GeneratorConfig {
        &self.config
    }

    /// Updates the configuration.
    ///
    /// This will reset the generator with the new configuration.
    ///
    /// # Panics
    /// Panics if the new configuration is invalid.
    pub fn set_config(&mut self, config: GeneratorConfig) {
        config.validate().expect("Invalid configuration");
        self.config = config.clone();
        self.walker = RandomWalkGenerator::new(config);
    }

    /// Helper to get current timestamp.
    fn get_current_timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    }
}

impl Default for MarketDataGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pattern for MarketDataGenerator
pub struct GeneratorBuilder {
    config: GeneratorConfig,
}

impl GeneratorBuilder {
    /// Creates a new generator builder.
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
        }
    }

    /// Sets the configuration.
    pub fn config(mut self, config: GeneratorConfig) -> Self {
        self.config = config;
        self
    }

    /// Builds the generator.
    ///
    /// # Panics
    /// Panics if the configuration is invalid.
    pub fn build(self) -> MarketDataGenerator {
        MarketDataGenerator::with_config(self.config)
    }
}

impl Default for GeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = MarketDataGenerator::new();
        assert_eq!(generator.config().starting_price, 100.0);
    }

    #[test]
    fn test_generator_with_config() {
        let config = GeneratorConfig::builder()
            .starting_price(50.0)
            .volatility(0.05)
            .build();
        
        let generator = MarketDataGenerator::with_config(config);
        assert_eq!(generator.config().starting_price, 50.0);
        assert_eq!(generator.config().volatility, 0.05);
    }

    #[test]
    fn test_generate_series() {
        let mut generator = MarketDataGenerator::new();
        generator.set_seed(42); // For reproducibility
        
        let candles = generator.generate_series(5);
        assert_eq!(candles.len(), 5);
        
        // Verify timestamps are increasing
        for i in 1..candles.len() {
            assert!(candles[i].timestamp > candles[i-1].timestamp);
        }
    }

    #[test]
    fn test_generate_ticks() {
        let mut generator = MarketDataGenerator::new();
        generator.set_seed(42);
        
        let ticks = generator.generate_ticks(10);
        assert_eq!(ticks.len(), 10);
        
        // Verify all ticks have valid data
        for tick in &ticks {
            assert!(tick.price > 0.0);
            assert!(tick.volume > 0);
        }
    }

    #[test]
    fn test_reproducibility() {
        let config = GeneratorConfig::builder()
            .seed(123)
            .build();
        
        let mut gen1 = MarketDataGenerator::with_config(config.clone());
        let mut gen2 = MarketDataGenerator::with_config(config);
        
        let candles1 = gen1.generate_series(3);
        let candles2 = gen2.generate_series(3);
        
        for i in 0..3 {
            assert_eq!(candles1[i].open, candles2[i].open);
            assert_eq!(candles1[i].close, candles2[i].close);
        }
    }

    #[test]
    fn test_reset() {
        let mut generator = MarketDataGenerator::new();
        generator.set_seed(42);
        
        let candle1 = generator.generate_candle();
        generator.reset();
        let candle2 = generator.generate_candle();
        
        // After reset, should generate same data with same seed
        assert_eq!(candle1.open, candle2.open);
    }

    #[test]
    fn test_builder_pattern() {
        let generator = GeneratorBuilder::new()
            .config(GeneratorConfig::volatile())
            .build();
        
        assert_eq!(generator.config().volatility, 0.05);
    }
}