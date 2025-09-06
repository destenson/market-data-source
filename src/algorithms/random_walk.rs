//! Random walk with drift algorithm for price generation

use rand::Rng;
use rand_distr::{Distribution, Normal};
use crate::config::{GeneratorConfig, TrendDirection};

/// Random walk generator for market prices
pub struct RandomWalkGenerator {
    /// Current price in the walk
    current_price: f64,
    /// Configuration for generation
    config: GeneratorConfig,
    /// Normal distribution for price changes
    price_distribution: Normal<f64>,
    /// Normal distribution for volume
    volume_distribution: Normal<f64>,
}

impl RandomWalkGenerator {
    /// Creates a new random walk generator
    pub fn new(config: GeneratorConfig) -> Result<Self, String> {
        // Create normal distribution for price changes
        let price_distribution = Normal::new(0.0, config.volatility)
            .map_err(|e| format!("Failed to create price distribution: {}", e))?;
        
        // Create normal distribution for volume
        let volume_distribution = Normal::new(
            config.base_volume as f64,
            config.base_volume as f64 * config.volume_volatility
        ).map_err(|e| format!("Failed to create volume distribution: {}", e))?;

        Ok(Self {
            current_price: config.starting_price,
            config,
            price_distribution,
            volume_distribution,
        })
    }

    /// Generates the next price in the random walk
    pub fn next_price<R: Rng>(&mut self, rng: &mut R) -> f64 {
        // Calculate drift based on trend
        let drift = match self.config.trend_direction {
            TrendDirection::Bullish => self.config.trend_strength,
            TrendDirection::Bearish => -self.config.trend_strength,
            TrendDirection::Sideways => 0.0,
        };

        // Generate random change
        let random_change = self.price_distribution.sample(rng);
        
        // Calculate price change as percentage
        let price_change = self.current_price * (drift + random_change);
        
        // Update price
        self.current_price += price_change;
        
        // Apply boundaries
        self.current_price = self.current_price
            .max(self.config.min_price)
            .min(self.config.max_price);
        
        self.current_price
    }

    /// Generates OHLC values from multiple price points
    pub fn generate_ohlc<R: Rng>(&mut self, rng: &mut R, num_ticks: usize) -> (f64, f64, f64, f64) {
        if num_ticks == 0 {
            let price = self.current_price;
            return (price, price, price, price);
        }

        let open = self.current_price;
        let mut high = open;
        let mut low = open;
        
        // Generate intermediate prices
        for _ in 0..num_ticks {
            let price = self.next_price(rng);
            high = high.max(price);
            low = low.min(price);
        }
        
        let close = self.current_price;
        
        (open, high, low, close)
    }

    /// Generates a volume value
    pub fn generate_volume<R: Rng>(&mut self, rng: &mut R) -> u64 {
        let volume = self.volume_distribution.sample(rng);
        volume.max(0.0) as u64
    }

    /// Resets the generator to starting price
    pub fn reset(&mut self) {
        self.current_price = self.config.starting_price;
    }

    /// Gets the current price
    pub fn current_price(&self) -> f64 {
        self.current_price
    }

    /// Sets the current price
    pub fn set_price(&mut self, price: f64) {
        self.current_price = price;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_random_walk_creation() {
        let config = GeneratorConfig::default();
        let generator = RandomWalkGenerator::new(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_price_generation() {
        let mut config = GeneratorConfig::default();
        config.seed = Some(42);
        config.volatility = 0.01;
        
        let mut generator = RandomWalkGenerator::new(config).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        
        let price1 = generator.next_price(&mut rng);
        let price2 = generator.next_price(&mut rng);
        
        assert!(price1 > 0.0);
        assert!(price2 > 0.0);
        assert_ne!(price1, price2); // Prices should change
    }

    #[test]
    fn test_bullish_trend() {
        let mut config = GeneratorConfig::default();
        config.seed = Some(42);
        config.trend_direction = TrendDirection::Bullish;
        config.trend_strength = 0.01;
        config.volatility = 0.001; // Low volatility to see trend clearly
        config.starting_price = 100.0;
        
        let mut generator = RandomWalkGenerator::new(config).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        
        let start_price = generator.current_price();
        
        // Generate many prices to see trend
        for _ in 0..100 {
            generator.next_price(&mut rng);
        }
        
        let end_price = generator.current_price();
        
        // With bullish trend, end price should generally be higher
        assert!(end_price > start_price);
    }

    #[test]
    fn test_price_boundaries() {
        let mut config = GeneratorConfig::default();
        config.min_price = 50.0;
        config.max_price = 150.0;
        config.starting_price = 100.0;
        config.volatility = 0.5; // High volatility to test boundaries
        
        let mut generator = RandomWalkGenerator::new(config).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        
        for _ in 0..1000 {
            let price = generator.next_price(&mut rng);
            assert!(price >= 50.0);
            assert!(price <= 150.0);
        }
    }

    #[test]
    fn test_ohlc_generation() {
        let config = GeneratorConfig::default();
        let mut generator = RandomWalkGenerator::new(config).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        
        let (open, high, low, close) = generator.generate_ohlc(&mut rng, 10);
        
        assert!(high >= open);
        assert!(high >= close);
        assert!(low <= open);
        assert!(low <= close);
        assert!(high >= low);
    }

    #[test]
    fn test_volume_generation() {
        let mut config = GeneratorConfig::default();
        config.base_volume = 100000;
        config.volume_volatility = 0.2;
        
        let mut generator = RandomWalkGenerator::new(config).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        
        let volume = generator.generate_volume(&mut rng);
        assert!(volume > 0);
    }
}