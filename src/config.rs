//! Configuration structures for market data generation

use crate::types::TimeInterval;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[cfg(feature = "serde")]
fn serialize_f64_inf<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_infinite() {
        serializer.serialize_none()
    } else {
        serializer.serialize_f64(*value)
    }
}

#[cfg(feature = "serde")]
fn deserialize_f64_inf<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<f64> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(f64::INFINITY))
}

/// Direction of market trend
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TrendDirection {
    /// Upward trend (bullish)
    Bullish,
    /// Downward trend (bearish)
    Bearish,
    /// No clear trend (sideways/ranging)
    Sideways,
}

/// Configuration for market data generation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeneratorConfig {
    /// Starting price for generation
    pub starting_price: f64,
    /// Minimum price boundary
    pub min_price: f64,
    /// Maximum price boundary
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_f64_inf"))]
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_f64_inf"))]
    pub max_price: f64,
    /// Trend direction
    pub trend_direction: TrendDirection,
    /// Trend strength as percentage per period (e.g., 0.01 = 1% per period)
    pub trend_strength: f64,
    /// Volatility (standard deviation of price changes)
    pub volatility: f64,
    /// Time interval for each data point
    pub time_interval: TimeInterval,
    /// Number of data points to generate
    pub num_points: usize,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
    /// Base volume for generation
    pub base_volume: u64,
    /// Volume volatility (standard deviation)
    pub volume_volatility: f64,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            starting_price: 100.0,
            min_price: 1.0,
            max_price: f64::INFINITY,
            trend_direction: TrendDirection::Sideways,
            trend_strength: 0.0,
            volatility: 0.02, // 2% volatility
            time_interval: TimeInterval::OneMinute,
            num_points: 100,
            seed: None,
            base_volume: 100000,
            volume_volatility: 0.3, // 30% volume volatility
        }
    }
}

impl GeneratorConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder for fluent configuration
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.starting_price <= 0.0 {
            return Err(ConfigError::InvalidPrice("Starting price must be positive".into()));
        }
        if self.min_price <= 0.0 {
            return Err(ConfigError::InvalidPrice("Minimum price must be positive".into()));
        }
        if self.min_price >= self.max_price {
            return Err(ConfigError::InvalidPrice("Minimum price must be less than maximum price".into()));
        }
        if self.volatility < 0.0 {
            return Err(ConfigError::InvalidVolatility("Volatility must be non-negative".into()));
        }
        if self.trend_strength < -1.0 || self.trend_strength > 1.0 {
            return Err(ConfigError::InvalidTrend("Trend strength must be between -100% and +100%".into()));
        }
        if self.num_points == 0 {
            return Err(ConfigError::InvalidParameter("Number of points must be positive".into()));
        }
        if self.base_volume == 0 {
            return Err(ConfigError::InvalidParameter("Base volume must be positive".into()));
        }
        if self.volume_volatility < 0.0 {
            return Err(ConfigError::InvalidVolatility("Volume volatility must be non-negative".into()));
        }
        Ok(())
    }
}

/// Builder for GeneratorConfig with fluent API
pub struct ConfigBuilder {
    config: GeneratorConfig,
}

impl ConfigBuilder {
    /// Creates a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
        }
    }

    /// Sets the starting price
    pub fn starting_price(mut self, price: f64) -> Self {
        self.config.starting_price = price;
        self
    }

    /// Sets the price boundaries
    pub fn price_range(mut self, min: f64, max: f64) -> Self {
        self.config.min_price = min;
        self.config.max_price = max;
        self
    }

    /// Sets the trend direction and strength
    pub fn trend(mut self, direction: TrendDirection, strength: f64) -> Self {
        self.config.trend_direction = direction;
        self.config.trend_strength = strength;
        self
    }

    /// Sets the volatility
    pub fn volatility(mut self, volatility: f64) -> Self {
        self.config.volatility = volatility;
        self
    }

    /// Sets the time interval
    pub fn time_interval(mut self, interval: TimeInterval) -> Self {
        self.config.time_interval = interval;
        self
    }

    /// Sets the number of points to generate
    pub fn num_points(mut self, num: usize) -> Self {
        self.config.num_points = num;
        self
    }

    /// Sets the random seed for reproducibility
    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    /// Sets the base volume
    pub fn base_volume(mut self, volume: u64) -> Self {
        self.config.base_volume = volume;
        self
    }

    /// Sets the volume volatility
    pub fn volume_volatility(mut self, volatility: f64) -> Self {
        self.config.volume_volatility = volatility;
        self
    }

    /// Builds the configuration, validating all parameters
    pub fn build(self) -> Result<GeneratorConfig, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

/// Error type for configuration validation
#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidPrice(String),
    InvalidVolatility(String),
    InvalidTrend(String),
    InvalidParameter(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidPrice(msg) => write!(f, "Invalid price configuration: {}", msg),
            ConfigError::InvalidVolatility(msg) => write!(f, "Invalid volatility configuration: {}", msg),
            ConfigError::InvalidTrend(msg) => write!(f, "Invalid trend configuration: {}", msg),
            ConfigError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Preset configurations for common scenarios
impl GeneratorConfig {
    /// Creates a configuration for a volatile market
    pub fn volatile() -> Self {
        Self {
            volatility: 0.05, // 5% volatility
            volume_volatility: 0.5, // 50% volume volatility
            ..Self::default()
        }
    }

    /// Creates a configuration for a stable market
    pub fn stable() -> Self {
        Self {
            volatility: 0.005, // 0.5% volatility
            volume_volatility: 0.1, // 10% volume volatility
            ..Self::default()
        }
    }

    /// Creates a configuration for a trending bull market
    pub fn bull_market() -> Self {
        Self {
            trend_direction: TrendDirection::Bullish,
            trend_strength: 0.002, // 0.2% per period
            volatility: 0.02,
            ..Self::default()
        }
    }

    /// Creates a configuration for a trending bear market
    pub fn bear_market() -> Self {
        Self {
            trend_direction: TrendDirection::Bearish,
            trend_strength: 0.002, // 0.2% per period
            volatility: 0.03, // Slightly higher volatility in bear markets
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GeneratorConfig::default();
        assert_eq!(config.starting_price, 100.0);
        assert_eq!(config.min_price, 1.0);
        assert_eq!(config.trend_direction, TrendDirection::Sideways);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = GeneratorConfig::builder()
            .starting_price(50.0)
            .price_range(10.0, 200.0)
            .trend(TrendDirection::Bullish, 0.01)
            .volatility(0.03)
            .num_points(500)
            .seed(42)
            .build()
            .unwrap();

        assert_eq!(config.starting_price, 50.0);
        assert_eq!(config.min_price, 10.0);
        assert_eq!(config.max_price, 200.0);
        assert_eq!(config.trend_direction, TrendDirection::Bullish);
        assert_eq!(config.trend_strength, 0.01);
        assert_eq!(config.volatility, 0.03);
        assert_eq!(config.num_points, 500);
        assert_eq!(config.seed, Some(42));
    }

    #[test]
    fn test_config_validation() {
        // Invalid starting price
        let mut config = GeneratorConfig::default();
        config.starting_price = -10.0;
        assert!(config.validate().is_err());

        // Invalid price range
        config = GeneratorConfig::default();
        config.min_price = 100.0;
        config.max_price = 50.0;
        assert!(config.validate().is_err());

        // Invalid volatility
        config = GeneratorConfig::default();
        config.volatility = -0.1;
        assert!(config.validate().is_err());

        // Invalid trend strength
        config = GeneratorConfig::default();
        config.trend_strength = 1.5;
        assert!(config.validate().is_err());

        // Zero points
        config = GeneratorConfig::default();
        config.num_points = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_preset_configs() {
        let volatile = GeneratorConfig::volatile();
        assert_eq!(volatile.volatility, 0.05);
        assert!(volatile.validate().is_ok());

        let stable = GeneratorConfig::stable();
        assert_eq!(stable.volatility, 0.005);
        assert!(stable.validate().is_ok());

        let bull = GeneratorConfig::bull_market();
        assert_eq!(bull.trend_direction, TrendDirection::Bullish);
        assert!(bull.validate().is_ok());

        let bear = GeneratorConfig::bear_market();
        assert_eq!(bear.trend_direction, TrendDirection::Bearish);
        assert!(bear.validate().is_ok());
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_trend_direction_serialization() {
            let trend = TrendDirection::Bullish;
            let json = serde_json::to_string(&trend).unwrap();
            assert_eq!(json, r#""bullish""#);
            
            let deserialized: TrendDirection = serde_json::from_str(&json).unwrap();
            assert_eq!(trend, deserialized);
        }

        #[test]
        fn test_generator_config_serialization() {
            let config = GeneratorConfig::builder()
                .starting_price(50.0)
                .price_range(10.0, 200.0)
                .trend(TrendDirection::Bullish, 0.01)
                .volatility(0.03)
                .num_points(500)
                .seed(42)
                .base_volume(100000)
                .volume_volatility(0.3)
                .time_interval(TimeInterval::FiveMinutes)
                .build()
                .unwrap();

            // Serialize to JSON
            let json = serde_json::to_string(&config).unwrap();
            
            // Deserialize back
            let deserialized: GeneratorConfig = serde_json::from_str(&json).unwrap();
            
            // Check key fields match
            assert_eq!(config.starting_price, deserialized.starting_price);
            assert_eq!(config.min_price, deserialized.min_price);
            assert_eq!(config.max_price, deserialized.max_price);
            assert_eq!(config.trend_direction, deserialized.trend_direction);
            assert_eq!(config.trend_strength, deserialized.trend_strength);
            assert_eq!(config.volatility, deserialized.volatility);
            assert_eq!(config.num_points, deserialized.num_points);
            assert_eq!(config.seed, deserialized.seed);
            assert_eq!(config.base_volume, deserialized.base_volume);
            assert_eq!(config.time_interval, deserialized.time_interval);
        }

        #[test]
        fn test_config_json_format() {
            let config = GeneratorConfig::default();
            let json = serde_json::to_string_pretty(&config).unwrap();
            
            // Verify JSON can be parsed back
            let _: GeneratorConfig = serde_json::from_str(&json).unwrap();
            
            // JSON should contain expected fields
            assert!(json.contains("starting_price"));
            assert!(json.contains("trend_direction"));
            assert!(json.contains("volatility"));
        }
    }
}