//! Configuration structures for market data generation

use crate::types::TimeInterval;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[cfg(feature = "api-server")]
use utoipa::ToSchema;

#[cfg(feature = "serde")]
fn serialize_decimal_inf<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // For Decimal, we use a very large number to represent "infinity"
    let max_decimal = Decimal::from_f64(1e15).expect("1e15 should always convert to Decimal");
    if *value >= max_decimal {
        serializer.serialize_none()
    } else {
        Serialize::serialize(value, serializer)
    }
}

#[cfg(feature = "serde")]
fn deserialize_decimal_inf<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Decimal> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(|| Decimal::from_f64(1e15).expect("1e15 should always convert to Decimal")))
}

/// Direction of market trend
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub enum TrendDirection {
    #[cfg_attr(feature = "serde", serde(alias = "up"))]
    /// Upward trend (bullish)
    Bullish,
    #[cfg_attr(feature = "serde", serde(alias = "down"))]
    /// Downward trend (bearish)
    Bearish,
    #[cfg_attr(feature = "serde", serde(alias = "flat"))]
    /// No clear trend (sideways/ranging)
    Sideways,
}

/// Configuration for market data generation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub struct GeneratorConfig {
    /// Starting price for generation
    #[cfg_attr(feature = "serde", serde(default = "default_starting_price"))]
    pub starting_price: Decimal,
    /// Minimum price boundary
    #[cfg_attr(feature = "serde", serde(default = "default_min_price"))]
    pub min_price: Decimal,
    /// Maximum price boundary
    #[cfg_attr(feature = "serde", serde(default = "default_max_price"))]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_decimal_inf"))]
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_decimal_inf"))]
    pub max_price: Decimal,
    /// Trend direction
    #[cfg_attr(feature = "serde", serde(default = "default_trend_direction"))]
    pub trend_direction: TrendDirection,
    /// Trend strength as percentage per period (e.g., 0.01 = 1% per period)
    #[cfg_attr(feature = "serde", serde(default = "default_trend_strength"))]
    pub trend_strength: Decimal,
    /// Volatility (standard deviation of price changes)
    #[cfg_attr(feature = "serde", serde(default = "default_volatility"))]
    pub volatility: Decimal,
    /// Time interval for each data point
    #[cfg_attr(feature = "serde", serde(default = "default_time_interval"))]
    pub time_interval: TimeInterval,
    /// Number of data points to generate
    #[cfg_attr(feature = "serde", serde(default = "default_num_points"))]
    pub num_points: usize,
    /// Random seed for reproducibility
    #[cfg_attr(feature = "serde", serde(default))]
    pub seed: Option<u64>,
    /// Base volume for generation
    #[cfg_attr(feature = "serde", serde(default = "default_base_volume"))]
    pub base_volume: u64,
    /// Volume volatility (standard deviation)
    #[cfg_attr(feature = "serde", serde(default = "default_volume_volatility"))]
    pub volume_volatility: f64,
}

// Default value functions for serde
fn default_starting_price() -> Decimal {
    Decimal::from_f64(100.0).expect("100.0 should always convert to Decimal")
}

fn default_min_price() -> Decimal {
    Decimal::from_f64(1.0).expect("1.0 should always convert to Decimal")
}

fn default_max_price() -> Decimal {
    Decimal::from_f64(1e15).expect("1e15 should always convert to Decimal")
}

fn default_trend_direction() -> TrendDirection {
    TrendDirection::Sideways
}

fn default_trend_strength() -> Decimal {
    Decimal::ZERO
}

fn default_volatility() -> Decimal {
    Decimal::from_f64(0.02).expect("0.02 should always convert to Decimal")
}

fn default_time_interval() -> TimeInterval {
    TimeInterval::OneMinute
}

fn default_num_points() -> usize {
    100
}

fn default_base_volume() -> u64 {
    100000
}

fn default_volume_volatility() -> f64 {
    0.3
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            starting_price: default_starting_price(),
            min_price: default_min_price(),
            max_price: default_max_price(),
            trend_direction: default_trend_direction(),
            trend_strength: default_trend_strength(),
            volatility: default_volatility(),
            time_interval: default_time_interval(),
            num_points: default_num_points(),
            seed: None,
            base_volume: default_base_volume(),
            volume_volatility: default_volume_volatility(),
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

    /// Smart defaults: adjusts configuration based on provided values
    /// This ensures reasonable defaults when only partial config is provided
    pub fn apply_smart_defaults(&mut self) {
        // If min_price is still at default but starting_price is high, adjust min_price
        if self.min_price == default_min_price() && self.starting_price > Decimal::from(1000) {
            self.min_price = self.starting_price * Decimal::from_f64(0.01).expect("0.01 should always convert to Decimal"); // 1% of starting price
        }
        
        // If max_price is still at default but starting_price is set, adjust max_price
        if self.max_price == default_max_price() && self.starting_price != default_starting_price() {
            self.max_price = self.starting_price * Decimal::from(100); // 100x starting price
        }
        
        // Ensure min < starting < max
        if self.min_price >= self.starting_price {
            self.min_price = self.starting_price * Decimal::from_f64(0.5).expect("0.5 should always convert to Decimal");
        }
        if self.max_price <= self.starting_price {
            self.max_price = self.starting_price * Decimal::from(2);
        }
        
        // Adjust volatility based on asset type (inferred from price)
        if self.volatility == default_volatility() {
            if self.starting_price > Decimal::from(10000) {
                // Likely crypto (BTC, ETH)
                self.volatility = Decimal::from_f64(0.05).expect("0.05 should always convert to Decimal"); // 5% volatility
            } else if self.starting_price < Decimal::from(10) {
                // Likely forex or penny stocks
                self.volatility = Decimal::from_f64(0.005).expect("0.005 should always convert to Decimal"); // 0.5% volatility
            }
        }
        
        // If trend direction is up/down but strength is zero, set a reasonable strength
        if self.trend_strength == Decimal::ZERO {
            match self.trend_direction {
                TrendDirection::Bullish => self.trend_strength = Decimal::from_f64(0.0001).expect("0.0001 should always convert to Decimal"),
                TrendDirection::Bearish => self.trend_strength = Decimal::from_f64(-0.0001).expect("-0.0001 should always convert to Decimal"),
                TrendDirection::Sideways => {}
            }
        }
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.starting_price <= Decimal::ZERO {
            return Err(ConfigError::InvalidPrice("Starting price must be positive".into()));
        }
        if self.min_price <= Decimal::ZERO {
            return Err(ConfigError::InvalidPrice("Minimum price must be positive".into()));
        }
        if self.min_price >= self.max_price {
            return Err(ConfigError::InvalidPrice("Minimum price must be less than maximum price".into()));
        }
        if self.volatility < Decimal::ZERO {
            return Err(ConfigError::InvalidVolatility("Volatility must be non-negative".into()));
        }
        let one = Decimal::ONE;
        if self.trend_strength < -one || self.trend_strength > one {
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

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    /// Creates a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
        }
    }

    /// Sets the starting price
    pub fn starting_price(mut self, price: Decimal) -> Self {
        self.config.starting_price = price;
        self
    }

    /// Sets the starting price from f64 (convenience method)
    pub fn starting_price_f64(mut self, price: f64) -> Self {
        self.config.starting_price = Decimal::from_f64(price)
            .unwrap_or_else(|_| Decimal::from_f64(100.0).expect("100.0 should always convert to Decimal"));
        self
    }

    /// Sets the price boundaries
    pub fn price_range(mut self, min: Decimal, max: Decimal) -> Self {
        self.config.min_price = min;
        self.config.max_price = max;
        self
    }

    /// Sets the price boundaries from f64 (convenience method)
    pub fn price_range_f64(mut self, min: f64, max: f64) -> Self {
        self.config.min_price = Decimal::from_f64(min)
            .unwrap_or_else(|_| Decimal::from_f64(1.0).expect("1.0 should always convert to Decimal"));
        self.config.max_price = Decimal::from_f64(max)
            .unwrap_or_else(|_| Decimal::from_f64(1e15).expect("1e15 should always convert to Decimal"));
        self
    }

    /// Sets the trend direction and strength
    pub fn trend(mut self, direction: TrendDirection, strength: Decimal) -> Self {
        self.config.trend_direction = direction;
        self.config.trend_strength = strength;
        self
    }

    /// Sets the trend direction and strength from f64 (convenience method)
    pub fn trend_f64(mut self, direction: TrendDirection, strength: f64) -> Self {
        self.config.trend_direction = direction;
        self.config.trend_strength = Decimal::from_f64(strength).unwrap_or(Decimal::ZERO);
        self
    }

    /// Sets the volatility
    pub fn volatility(mut self, volatility: Decimal) -> Self {
        self.config.volatility = volatility;
        self
    }

    /// Sets the volatility from f64 (convenience method)
    pub fn volatility_f64(mut self, volatility: f64) -> Self {
        self.config.volatility = Decimal::from_f64(volatility)
            .unwrap_or_else(|_| Decimal::from_f64(0.02).expect("0.02 should always convert to Decimal"));
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
            ConfigError::InvalidPrice(msg) => write!(f, "Invalid price configuration: {msg}"),
            ConfigError::InvalidVolatility(msg) => write!(f, "Invalid volatility configuration: {msg}"),
            ConfigError::InvalidTrend(msg) => write!(f, "Invalid trend configuration: {msg}"),
            ConfigError::InvalidParameter(msg) => write!(f, "Invalid parameter: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Preset configurations for common scenarios
impl GeneratorConfig {
    /// Creates a configuration for a volatile market
    pub fn volatile() -> Self {
        Self {
            volatility: Decimal::from_f64(0.05).expect("0.05 should always convert to Decimal"), // 5% volatility
            volume_volatility: 0.5, // 50% volume volatility
            ..Self::default()
        }
    }

    /// Creates a configuration for a stable market
    pub fn stable() -> Self {
        Self {
            volatility: Decimal::from_f64(0.005).expect("0.005 should always convert to Decimal"), // 0.5% volatility
            volume_volatility: 0.1, // 10% volume volatility
            ..Self::default()
        }
    }

    /// Creates a configuration for a trending bull market
    pub fn bull_market() -> Self {
        Self {
            trend_direction: TrendDirection::Bullish,
            trend_strength: Decimal::from_f64(0.002).expect("0.002 should always convert to Decimal"), // 0.2% per period
            volatility: Decimal::from_f64(0.02).expect("0.02 should always convert to Decimal"),
            ..Self::default()
        }
    }

    /// Creates a configuration for a trending bear market
    pub fn bear_market() -> Self {
        Self {
            trend_direction: TrendDirection::Bearish,
            trend_strength: Decimal::from_f64(0.002).expect("0.002 should always convert to Decimal"), // 0.2% per period
            volatility: Decimal::from_f64(0.03).expect("0.03 should always convert to Decimal"), // Slightly higher volatility in bear markets
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
        assert_eq!(config.starting_price, Decimal::from_f64(100.0).unwrap());
        assert_eq!(config.min_price, Decimal::from_f64(1.0).unwrap());
        assert_eq!(config.trend_direction, TrendDirection::Sideways);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = GeneratorConfig::builder()
            .starting_price_f64(50.0)
            .price_range_f64(10.0, 200.0)
            .trend_f64(TrendDirection::Bullish, 0.01)
            .volatility_f64(0.03)
            .num_points(500)
            .seed(42)
            .build()
            .unwrap();

        assert_eq!(config.starting_price, Decimal::from_f64(50.0).unwrap());
        assert_eq!(config.min_price, Decimal::from_f64(10.0).unwrap());
        assert_eq!(config.max_price, Decimal::from_f64(200.0).unwrap());
        assert_eq!(config.trend_direction, TrendDirection::Bullish);
        assert_eq!(config.trend_strength, Decimal::from_f64(0.01).unwrap());
        assert_eq!(config.volatility, Decimal::from_f64(0.03).unwrap());
        assert_eq!(config.num_points, 500);
        assert_eq!(config.seed, Some(42));
    }

    #[test]
    fn test_config_validation() {
        // Invalid starting price
        let mut config = GeneratorConfig::default();
        config.starting_price = Decimal::from_f64(-10.0).unwrap();
        assert!(config.validate().is_err());

        // Invalid price range
        config = GeneratorConfig::default();
        config.min_price = Decimal::from_f64(100.0).unwrap();
        config.max_price = Decimal::from_f64(50.0).unwrap();
        assert!(config.validate().is_err());

        // Invalid volatility
        config = GeneratorConfig::default();
        config.volatility = Decimal::from_f64(-0.1).unwrap();
        assert!(config.validate().is_err());

        // Invalid trend strength
        config = GeneratorConfig::default();
        config.trend_strength = Decimal::from_f64(1.5).unwrap();
        assert!(config.validate().is_err());

        // Zero points
        config = GeneratorConfig::default();
        config.num_points = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_preset_configs() {
        let volatile = GeneratorConfig::volatile();
        assert_eq!(volatile.volatility, Decimal::from_f64(0.05).unwrap());
        assert!(volatile.validate().is_ok());

        let stable = GeneratorConfig::stable();
        assert_eq!(stable.volatility, Decimal::from_f64(0.005).unwrap());
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
                .starting_price_f64(50.0)
                .price_range_f64(10.0, 200.0)
                .trend_f64(TrendDirection::Bullish, 0.01)
                .volatility_f64(0.03)
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
