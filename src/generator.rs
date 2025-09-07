//! Main market data generator

use rand::SeedableRng;
use rand::rngs::StdRng;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::algorithms::RandomWalkGenerator;
use crate::config::GeneratorConfig;
use crate::types::{OHLC, Tick};
use std::time::{SystemTime, UNIX_EPOCH};

/// Main market data generator
pub struct MarketDataGenerator {
    /// Random number generator
    rng: StdRng,
    /// Configuration
    config: GeneratorConfig,
    /// Price generator algorithm
    price_generator: RandomWalkGenerator,
    /// Current timestamp in milliseconds
    current_timestamp: i64,
}

impl MarketDataGenerator {
    /// Creates a new generator with default configuration
    pub fn new() -> Self {
        Self::with_config(GeneratorConfig::default()).expect("Default config should be valid")
    }

    /// Creates a new generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Result<Self, String> {
        // Validate configuration
        config.validate()
            .map_err(|e| format!("Invalid configuration: {e}"))?;

        // Create RNG with seed if provided
        let rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        // Create price generator
        let price_generator = RandomWalkGenerator::new(config.clone())?;

        // Get current timestamp
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get system time: {e}"))?
            .as_millis() as i64;

        Ok(Self {
            rng,
            config: config.clone(),
            price_generator,
            current_timestamp,
        })
    }

    /// Generates a single OHLC
    pub fn generate_ohlc(&mut self) -> OHLC {
        // Generate OHLC prices (using 10 ticks per candle for realism)
        let (open, high, low, close) = self.price_generator.generate_ohlc(&mut self.rng, 10);
        
        // Generate volume
        let volume = self.price_generator.generate_volume(&mut self.rng);
        
        // Get timestamp
        let timestamp = self.current_timestamp;
        
        // Advance timestamp for next candle
        self.current_timestamp += self.config.time_interval.millis() as i64;
        
        OHLC::new(open, high, low, close, volume, timestamp)
    }
    
    /// Generates a single OHLC candle
    #[deprecated(since = "0.2.0", note = "Use generate_ohlc() instead")]
    pub fn generate_candle(&mut self) -> OHLC {
        self.generate_ohlc()
    }

    /// Generates a series of OHLC candles
    pub fn generate_series(&mut self, count: usize) -> Vec<OHLC> {
        let mut candles = Vec::with_capacity(count);
        for _ in 0..count {
            candles.push(self.generate_ohlc());
        }
        candles
    }

    /// Generates a single tick
    pub fn generate_tick(&mut self) -> Tick {
        let price = self.price_generator.next_price(&mut self.rng);
        let volume = self.price_generator.generate_volume(&mut self.rng);
        let timestamp = self.current_timestamp;
        
        // Advance timestamp by 1 second for ticks
        self.current_timestamp += 1000;
        
        // Optionally generate bid/ask spread
        let spread = Decimal::from_f64(0.001).unwrap(); // 0.1% spread
        let half_spread = price * spread / Decimal::from(2);
        
        Tick::with_spread(
            price,
            volume,
            timestamp,
            price - half_spread,
            price + half_spread,
        )
    }

    /// Generates a series of ticks
    pub fn generate_ticks(&mut self, count: usize) -> Vec<Tick> {
        let mut ticks = Vec::with_capacity(count);
        for _ in 0..count {
            ticks.push(self.generate_tick());
        }
        ticks
    }

    /// Resets the generator to initial state
    pub fn reset(&mut self) {
        self.price_generator.reset();
        self.current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        
        // Reset RNG with seed if provided
        if let Some(seed) = self.config.seed {
            self.rng = StdRng::seed_from_u64(seed);
        }
    }

    /// Sets a specific starting timestamp
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.current_timestamp = timestamp;
    }

    /// Gets the current configuration
    pub fn config(&self) -> &GeneratorConfig {
        &self.config
    }

    /// Updates the configuration
    pub fn set_config(&mut self, config: GeneratorConfig) -> Result<(), String> {
        config.validate()
            .map_err(|e| format!("Invalid configuration: {e}"))?;
        
        // Update config
        self.config = config.clone();
        
        // Recreate price generator with new config
        self.price_generator = RandomWalkGenerator::new(config)?;
        
        // Update RNG if seed changed
        if let Some(seed) = self.config.seed {
            self.rng = StdRng::seed_from_u64(seed);
        }
        
        Ok(())
    }

    /// Generate OHLC data and export to CSV file
    #[cfg(feature = "csv_export")]
    pub fn generate_to_csv_ohlc<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.generate_series(count);
        crate::export::to_csv_ohlc(&data, path)?;
        Ok(())
    }

    /// Generate tick data and export to CSV file
    #[cfg(feature = "csv_export")]
    pub fn generate_to_csv_ticks<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.generate_ticks(count);
        crate::export::to_csv_ticks(&data, path)?;
        Ok(())
    }

    /// Stream generate OHLC data directly to CSV file (memory efficient for large datasets)
    #[cfg(feature = "csv_export")]
    pub fn stream_generate_to_csv_ohlc<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::export::csv::CsvExporter;
        
        let exporter = CsvExporter::default();
        
        // Create an iterator that generates candles on-the-fly
        let iter = (0..count).map(|_| self.generate_candle());
        
        Ok(exporter.stream_ohlc(iter, path)?)
    }

    /// Stream generate tick data directly to CSV file (memory efficient for large datasets)
    #[cfg(feature = "csv_export")]
    pub fn stream_generate_to_csv_ticks<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::export::csv::CsvExporter;
        
        let exporter = CsvExporter::default();
        
        // Create an iterator that generates ticks on-the-fly
        let iter = (0..count).map(|_| self.generate_tick());
        
        Ok(exporter.stream_ticks(iter, path)?)
    }
}

impl Default for MarketDataGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigBuilder;

    #[test]
    fn test_generator_creation() {
        let generator = MarketDataGenerator::new();
        assert_eq!(generator.config().starting_price, Decimal::from_f64(100.0).unwrap());
    }

    #[test]
    fn test_generator_with_config() {
        let config = ConfigBuilder::new()
            .starting_price_f64(50.0)
            .volatility_f64(0.03)
            .seed(42)
            .build()
            .unwrap();
        
        let generator = MarketDataGenerator::with_config(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_candle_generation() {
        let config = ConfigBuilder::new()
            .seed(42)
            .build()
            .unwrap();
        
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let candle = generator.generate_candle();
        
        assert!(candle.is_valid());
        assert!(candle.volume.value() > 0);
    }

    #[test]
    fn test_series_generation() {
        let config = ConfigBuilder::new()
            .seed(42)
            .num_points(10)
            .build()
            .unwrap();
        
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let series = generator.generate_series(10);
        
        assert_eq!(series.len(), 10);
        
        // Check timestamps are increasing
        for i in 1..series.len() {
            assert!(series[i].timestamp > series[i-1].timestamp);
        }
    }

    #[test]
    fn test_tick_generation() {
        let mut generator = MarketDataGenerator::new();
        let tick = generator.generate_tick();
        
        assert!(tick.price > Decimal::ZERO);
        assert!(tick.volume.value() > 0);
        assert!(tick.bid.is_some());
        assert!(tick.ask.is_some());
        
        if let (Some(bid), Some(ask)) = (tick.bid, tick.ask) {
            assert!(ask > bid); // Spread should be positive
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let config = ConfigBuilder::new()
            .seed(42)
            .build()
            .unwrap();
        
        let mut gen1 = MarketDataGenerator::with_config(config.clone()).unwrap();
        let mut gen2 = MarketDataGenerator::with_config(config).unwrap();
        
        let candles1 = gen1.generate_series(5);
        let candles2 = gen2.generate_series(5);
        
        // With same seed, should generate same data
        for (c1, c2) in candles1.iter().zip(candles2.iter()) {
            assert_eq!(c1.open, c2.open);
            assert_eq!(c1.close, c2.close);
        }
    }

    #[test]
    fn test_reset() {
        let config = ConfigBuilder::new()
            .seed(42)
            .starting_price_f64(100.0)
            .build()
            .unwrap();
        
        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        
        let candle1 = generator.generate_candle();
        generator.reset();
        let candle2 = generator.generate_candle();
        
        // After reset with same seed, should generate same values
        assert_eq!(candle1.open, candle2.open);
    }
}