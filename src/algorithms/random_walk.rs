//! Random walk with drift algorithm for price generation.

use crate::config::GeneratorConfig;
use crate::types::OHLC;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Normal};

/// Generator that implements random walk with drift for price movements.
pub struct RandomWalkGenerator {
    config: GeneratorConfig,
    rng: StdRng,
    current_price: f64,
    current_timestamp: i64,
}

impl RandomWalkGenerator {
    /// Creates a new random walk generator with the given configuration.
    pub fn new(config: GeneratorConfig) -> Self {
        let rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        let current_price = config.starting_price;
        let current_timestamp = 0; // Will be set properly when generating
        
        Self {
            config,
            rng,
            current_price,
            current_timestamp,
        }
    }

    /// Generates the next price using random walk with drift.
    ///
    /// The formula is: next_price = current_price * (1 + drift + volatility * N(0,1))
    pub fn next_price(&mut self) -> f64 {
        let drift = self.config.effective_drift();
        let volatility = self.config.volatility;
        
        // Generate random normal value
        let normal = Normal::new(0.0, 1.0).unwrap();
        let random_shock = normal.sample(&mut self.rng);
        
        // Calculate price change
        let return_rate = drift + volatility * random_shock;
        let mut new_price = self.current_price * (1.0 + return_rate);
        
        // Enforce price bounds
        new_price = new_price.max(self.config.min_price);
        if self.config.max_price != f64::INFINITY {
            new_price = new_price.min(self.config.max_price);
        }
        
        // Ensure price stays positive
        new_price = new_price.max(0.001);
        
        self.current_price = new_price;
        new_price
    }

    /// Generates a series of prices for a single candle period.
    ///
    /// Creates multiple intra-period prices to form realistic OHLC values.
    pub fn generate_candle_prices(&mut self, num_ticks: usize) -> Vec<f64> {
        let mut prices = Vec::with_capacity(num_ticks);
        
        for _ in 0..num_ticks {
            prices.push(self.next_price());
        }
        
        prices
    }

    /// Generates a single OHLC candle.
    pub fn generate_candle(&mut self) -> OHLC {
        // Generate multiple prices within the period for realistic OHLC
        let prices = self.generate_candle_prices(10); // 10 ticks per candle
        
        let open = prices[0];
        let close = prices[prices.len() - 1];
        let high = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let low = prices.iter().cloned().fold(f64::INFINITY, f64::min);
        
        // Generate volume with some randomness
        let volume = self.generate_volume();
        
        let timestamp = self.current_timestamp;
        self.current_timestamp += self.config.time_interval.as_millis() as i64;
        
        OHLC::new(open, high, low, close, volume, timestamp)
    }

    /// Generates volume with configured volatility.
    pub fn generate_volume(&mut self) -> u64 {
        let avg_volume = self.config.avg_volume as f64;
        let volume_volatility = self.config.volume_volatility;
        
        let normal = Normal::new(0.0, 1.0).unwrap();
        let random_factor = normal.sample(&mut self.rng);
        
        let volume = avg_volume * (1.0 + volume_volatility * random_factor);
        volume.max(1.0) as u64
    }

    /// Resets the generator to initial state.
    pub fn reset(&mut self) {
        self.current_price = self.config.starting_price;
        self.current_timestamp = 0;
        if let Some(seed) = self.config.seed {
            self.rng = StdRng::seed_from_u64(seed);
        }
    }

    /// Sets the starting timestamp for generation.
    pub fn set_start_timestamp(&mut self, timestamp: i64) {
        self.current_timestamp = timestamp;
    }
}

/// Generates OHLC from a series of price points.
pub fn generate_ohlc_from_prices(prices: &[f64], volume: u64, timestamp: i64) -> OHLC {
    assert!(!prices.is_empty(), "Cannot generate OHLC from empty prices");
    
    let open = prices[0];
    let close = prices[prices.len() - 1];
    let high = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let low = prices.iter().cloned().fold(f64::INFINITY, f64::min);
    
    OHLC::new(open, high, low, close, volume, timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TrendDirection;

    #[test]
    fn test_deterministic_generation() {
        let config = GeneratorConfig::builder()
            .seed(42)
            .starting_price(100.0)
            .build();
        
        let mut gen1 = RandomWalkGenerator::new(config.clone());
        let mut gen2 = RandomWalkGenerator::new(config);
        
        let price1 = gen1.next_price();
        let price2 = gen2.next_price();
        
        assert_eq!(price1, price2, "Same seed should produce same prices");
    }

    #[test]
    fn test_price_bounds() {
        let config = GeneratorConfig::builder()
            .starting_price(100.0)
            .min_price(90.0)
            .max_price(110.0)
            .volatility(0.5) // High volatility to test bounds
            .seed(42)
            .build();
        
        let mut generator = RandomWalkGenerator::new(config);
        
        for _ in 0..100 {
            let price = generator.next_price();
            assert!(price >= 90.0, "Price {} below minimum", price);
            assert!(price <= 110.0, "Price {} above maximum", price);
        }
    }

    #[test]
    fn test_upward_trend() {
        let config = GeneratorConfig::builder()
            .starting_price(100.0)
            .trend_direction(TrendDirection::Bullish)
            .trend_strength(10.0) // Strong trend
            .volatility(0.001) // Low volatility
            .seed(42)
            .build();
        
        let mut generator = RandomWalkGenerator::new(config);
        let start_price = 100.0;
        
        // Generate many prices
        for _ in 0..100 {
            generator.next_price();
        }
        
        let end_price = generator.current_price;
        assert!(end_price > start_price, "Bullish trend should increase price");
    }

    #[test]
    fn test_candle_generation() {
        let config = GeneratorConfig::builder()
            .starting_price(100.0)
            .seed(42)
            .build();
        
        let mut generator = RandomWalkGenerator::new(config);
        let candle = generator.generate_candle();
        
        // Verify OHLC relationships
        assert!(candle.high >= candle.open);
        assert!(candle.high >= candle.close);
        assert!(candle.low <= candle.open);
        assert!(candle.low <= candle.close);
        assert!(candle.volume > 0);
    }

    #[test]
    fn test_volume_generation() {
        let config = GeneratorConfig::builder()
            .avg_volume(10000)
            .volume_volatility(0.2)
            .seed(42)
            .build();
        
        let mut generator = RandomWalkGenerator::new(config);
        
        let mut total_volume = 0u64;
        for _ in 0..100 {
            total_volume += generator.generate_volume();
        }
        
        let avg = total_volume / 100;
        // Average should be close to configured value
        assert!(avg > 8000 && avg < 12000, "Average volume {} out of expected range", avg);
    }

    #[test]
    fn test_generate_ohlc_from_prices() {
        let prices = vec![100.0, 102.0, 98.0, 101.0, 99.0];
        let ohlc = generate_ohlc_from_prices(&prices, 1000, 123456);
        
        assert_eq!(ohlc.open, 100.0);
        assert_eq!(ohlc.close, 99.0);
        assert_eq!(ohlc.high, 102.0);
        assert_eq!(ohlc.low, 98.0);
        assert_eq!(ohlc.volume, 1000);
        assert_eq!(ohlc.timestamp, 123456);
    }
}