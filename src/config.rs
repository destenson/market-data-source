//! Configuration structures for market data generation.

use crate::types::TimeInterval;

/// Direction of the price trend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrendDirection {
    /// Upward price movement
    Bullish,
    /// Downward price movement
    Bearish,
    /// No clear direction
    Sideways,
    /// Custom trend with specific drift percentage
    Custom(f64),
}

impl TrendDirection {
    /// Returns the drift rate for this trend direction.
    pub fn drift_rate(&self) -> f64 {
        match self {
            Self::Bullish => 0.0005,    // 0.05% per period
            Self::Bearish => -0.0005,   // -0.05% per period
            Self::Sideways => 0.0,       // No drift
            Self::Custom(rate) => *rate,
        }
    }
}

/// Configuration for market data generation.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Starting price for generation
    pub starting_price: f64,
    /// Minimum price bound (use 0.0 for no lower bound)
    pub min_price: f64,
    /// Maximum price bound (use f64::INFINITY for no upper bound)
    pub max_price: f64,
    /// Trend direction and strength
    pub trend_direction: TrendDirection,
    /// Trend strength (multiplier for drift rate)
    pub trend_strength: f64,
    /// Volatility (standard deviation of price changes)
    pub volatility: f64,
    /// Time interval for each candle
    pub time_interval: TimeInterval,
    /// Number of data points to generate
    pub num_points: usize,
    /// Optional random seed for reproducibility
    pub seed: Option<u64>,
    /// Average volume per period
    pub avg_volume: u64,
    /// Volume volatility (standard deviation as percentage of average)
    pub volume_volatility: f64,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            starting_price: 100.0,
            min_price: 0.0,
            max_price: f64::INFINITY,
            trend_direction: TrendDirection::Sideways,
            trend_strength: 1.0,
            volatility: 0.02,  // 2% standard deviation
            time_interval: TimeInterval::OneMinute,
            num_points: 100,
            seed: None,
            avg_volume: 10000,
            volume_volatility: 0.3,  // 30% volume variation
        }
    }
}

impl GeneratorConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration builder.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Validates the configuration parameters.
    ///
    /// # Errors
    /// Returns an error if any parameters are invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.starting_price <= 0.0 {
            return Err("Starting price must be positive".to_string());
        }

        if self.min_price < 0.0 {
            return Err("Minimum price cannot be negative".to_string());
        }

        if self.max_price <= self.min_price {
            return Err("Maximum price must be greater than minimum price".to_string());
        }

        if self.starting_price < self.min_price || self.starting_price > self.max_price {
            return Err("Starting price must be within min/max bounds".to_string());
        }

        if self.volatility < 0.0 {
            return Err("Volatility cannot be negative".to_string());
        }

        if self.trend_strength < 0.0 {
            return Err("Trend strength cannot be negative".to_string());
        }

        if self.num_points == 0 {
            return Err("Number of points must be positive".to_string());
        }

        if self.avg_volume == 0 {
            return Err("Average volume must be positive".to_string());
        }

        if self.volume_volatility < 0.0 || self.volume_volatility > 1.0 {
            return Err("Volume volatility must be between 0 and 1".to_string());
        }

        Ok(())
    }

    /// Returns the effective drift rate combining trend direction and strength.
    pub fn effective_drift(&self) -> f64 {
        self.trend_direction.drift_rate() * self.trend_strength
    }
}

/// Builder for GeneratorConfig with fluent API.
pub struct ConfigBuilder {
    config: GeneratorConfig,
}

impl ConfigBuilder {
    /// Creates a new configuration builder with default values.
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
        }
    }

    /// Sets the starting price.
    pub fn starting_price(mut self, price: f64) -> Self {
        self.config.starting_price = price;
        self
    }

    /// Sets the minimum price bound.
    pub fn min_price(mut self, price: f64) -> Self {
        self.config.min_price = price;
        self
    }

    /// Sets the maximum price bound.
    pub fn max_price(mut self, price: f64) -> Self {
        self.config.max_price = price;
        self
    }

    /// Sets the price bounds.
    pub fn price_bounds(mut self, min: f64, max: f64) -> Self {
        self.config.min_price = min;
        self.config.max_price = max;
        self
    }

    /// Sets the trend direction.
    pub fn trend_direction(mut self, direction: TrendDirection) -> Self {
        self.config.trend_direction = direction;
        self
    }

    /// Sets the trend strength multiplier.
    pub fn trend_strength(mut self, strength: f64) -> Self {
        self.config.trend_strength = strength;
        self
    }

    /// Sets the volatility (standard deviation).
    pub fn volatility(mut self, volatility: f64) -> Self {
        self.config.volatility = volatility;
        self
    }

    /// Sets the time interval for candles.
    pub fn time_interval(mut self, interval: TimeInterval) -> Self {
        self.config.time_interval = interval;
        self
    }

    /// Sets the number of points to generate.
    pub fn num_points(mut self, count: usize) -> Self {
        self.config.num_points = count;
        self
    }

    /// Sets the random seed for reproducibility.
    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    /// Sets the average volume per period.
    pub fn avg_volume(mut self, volume: u64) -> Self {
        self.config.avg_volume = volume;
        self
    }

    /// Sets the volume volatility.
    pub fn volume_volatility(mut self, volatility: f64) -> Self {
        self.config.volume_volatility = volatility;
        self
    }

    /// Builds the configuration, validating all parameters.
    ///
    /// # Panics
    /// Panics if validation fails. Use `try_build()` for non-panicking version.
    pub fn build(self) -> GeneratorConfig {
        self.config.validate().expect("Invalid configuration");
        self.config
    }

    /// Attempts to build the configuration, returning an error if validation fails.
    pub fn try_build(self) -> Result<GeneratorConfig, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset configurations for common scenarios.
impl GeneratorConfig {
    /// Creates a configuration for volatile market conditions.
    pub fn volatile() -> Self {
        Self::builder()
            .volatility(0.05)  // 5% volatility
            .volume_volatility(0.5)
            .build()
    }

    /// Creates a configuration for stable market conditions.
    pub fn stable() -> Self {
        Self::builder()
            .volatility(0.005)  // 0.5% volatility
            .volume_volatility(0.1)
            .build()
    }

    /// Creates a configuration for a strong uptrend.
    pub fn bull_market() -> Self {
        Self::builder()
            .trend_direction(TrendDirection::Bullish)
            .trend_strength(2.0)
            .volatility(0.015)
            .build()
    }

    /// Creates a configuration for a strong downtrend.
    pub fn bear_market() -> Self {
        Self::builder()
            .trend_direction(TrendDirection::Bearish)
            .trend_strength(2.0)
            .volatility(0.02)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GeneratorConfig::default();
        assert_eq!(config.starting_price, 100.0);
        assert_eq!(config.volatility, 0.02);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = GeneratorConfig::builder()
            .starting_price(50.0)
            .volatility(0.03)
            .trend_direction(TrendDirection::Bullish)
            .seed(42)
            .build();

        assert_eq!(config.starting_price, 50.0);
        assert_eq!(config.volatility, 0.03);
        assert_eq!(config.trend_direction, TrendDirection::Bullish);
        assert_eq!(config.seed, Some(42));
    }

    #[test]
    fn test_config_validation() {
        // Invalid starting price
        let result = GeneratorConfig::builder()
            .starting_price(-10.0)
            .try_build();
        assert!(result.is_err());

        // Invalid price bounds
        let result = GeneratorConfig::builder()
            .min_price(100.0)
            .max_price(50.0)
            .try_build();
        assert!(result.is_err());

        // Invalid volatility
        let result = GeneratorConfig::builder()
            .volatility(-0.1)
            .try_build();
        assert!(result.is_err());
    }

    #[test]
    fn test_trend_direction() {
        assert_eq!(TrendDirection::Bullish.drift_rate(), 0.0005);
        assert_eq!(TrendDirection::Bearish.drift_rate(), -0.0005);
        assert_eq!(TrendDirection::Sideways.drift_rate(), 0.0);
        assert_eq!(TrendDirection::Custom(0.001).drift_rate(), 0.001);
    }

    #[test]
    fn test_preset_configs() {
        let volatile = GeneratorConfig::volatile();
        assert_eq!(volatile.volatility, 0.05);

        let stable = GeneratorConfig::stable();
        assert_eq!(stable.volatility, 0.005);

        let bull = GeneratorConfig::bull_market();
        assert_eq!(bull.trend_direction, TrendDirection::Bullish);
        assert_eq!(bull.trend_strength, 2.0);

        let bear = GeneratorConfig::bear_market();
        assert_eq!(bear.trend_direction, TrendDirection::Bearish);
    }

    #[test]
    fn test_effective_drift() {
        let config = GeneratorConfig::builder()
            .trend_direction(TrendDirection::Bullish)
            .trend_strength(3.0)
            .build();
        
        assert_eq!(config.effective_drift(), 0.0015); // 0.0005 * 3.0
    }
}