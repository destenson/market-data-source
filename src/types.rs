//! Core data types for market data representation

use std::fmt;
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "api-server")]
use utoipa::ToSchema;

/// Represents an OHLC (Open, High, Low, Close) candle
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub struct OHLC {
    /// Opening price of the period
    pub open: Decimal,
    /// Highest price during the period
    pub high: Decimal,
    /// Lowest price during the period
    pub low: Decimal,
    /// Closing price of the period
    pub close: Decimal,
    /// Volume traded during the period
    pub volume: Volume,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
}

impl OHLC {
    /// Creates a new OHLC candle
    pub fn new(open: Decimal, high: Decimal, low: Decimal, close: Decimal, volume: u64, timestamp: i64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume: Volume::new(volume),
            timestamp,
        }
    }

    /// Validates that the OHLC values are consistent
    pub fn is_valid(&self) -> bool {
        // High should be the highest value
        let max = self.open.max(self.close);
        let min = self.open.min(self.close);
        
        self.high >= max && self.low <= min && 
        self.high >= self.low &&
        self.open > Decimal::ZERO && self.high > Decimal::ZERO && 
        self.low > Decimal::ZERO && self.close > Decimal::ZERO
    }

    /// Returns the price range (high - low)
    pub fn range(&self) -> Decimal {
        self.high - self.low
    }

    /// Returns the body size (|close - open|)
    pub fn body(&self) -> Decimal {
        (self.close - self.open).abs()
    }

    /// Returns true if this is a green/bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
}

/// Represents a single tick of market data
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub struct Tick {
    /// Price of the tick
    pub price: Decimal,
    /// Volume of the tick
    pub volume: Volume,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
    /// Optional bid price
    pub bid: Option<Decimal>,
    /// Optional ask price
    pub ask: Option<Decimal>,
}

impl Tick {
    /// Creates a new tick with just price and volume
    pub fn new(price: Decimal, volume: u64, timestamp: i64) -> Self {
        Self {
            price,
            volume: Volume::new(volume),
            timestamp,
            bid: None,
            ask: None,
        }
    }

    /// Creates a new tick with bid/ask spread
    pub fn with_spread(price: Decimal, volume: u64, timestamp: i64, bid: Decimal, ask: Decimal) -> Self {
        Self {
            price,
            volume: Volume::new(volume),
            timestamp,
            bid: Some(bid),
            ask: Some(ask),
        }
    }

    /// Returns the spread if bid and ask are available
    pub fn spread(&self) -> Option<Decimal> {
        match (self.bid, self.ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }
}

/// Represents volume in the market
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub struct Volume {
    pub value: u64,
}

impl Volume {
    /// Creates a new Volume
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the volume value
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Returns volume as f64 for calculations
    pub fn as_f64(&self) -> f64 {
        self.value as f64
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Time intervals for candle periods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "api-server", derive(ToSchema))]
pub enum TimeInterval {
    /// One minute
    #[cfg_attr(feature = "serde", serde(rename = "1m"))]
    OneMinute,
    /// Five minutes
    #[cfg_attr(feature = "serde", serde(rename = "5m"))]
    FiveMinutes,
    /// Fifteen minutes
    #[cfg_attr(feature = "serde", serde(rename = "15m"))]
    FifteenMinutes,
    /// Thirty minutes
    #[cfg_attr(feature = "serde", serde(rename = "30m"))]
    ThirtyMinutes,
    /// One hour
    #[cfg_attr(feature = "serde", serde(rename = "1h"))]
    OneHour,
    /// Four hours
    #[cfg_attr(feature = "serde", serde(rename = "4h"))]
    FourHours,
    /// One day
    #[cfg_attr(feature = "serde", serde(rename = "1d"))]
    OneDay,
    /// Custom interval in seconds
    #[cfg_attr(feature = "serde", serde(rename = "custom"))]
    Custom(u32),
}

impl TimeInterval {
    /// Returns the interval duration in seconds
    pub fn seconds(&self) -> u32 {
        match self {
            TimeInterval::OneMinute => 60,
            TimeInterval::FiveMinutes => 300,
            TimeInterval::FifteenMinutes => 900,
            TimeInterval::ThirtyMinutes => 1800,
            TimeInterval::OneHour => 3600,
            TimeInterval::FourHours => 14400,
            TimeInterval::OneDay => 86400,
            TimeInterval::Custom(s) => *s,
        }
    }

    /// Returns the interval duration in milliseconds
    pub fn millis(&self) -> u64 {
        self.seconds() as u64 * 1000
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInterval::OneMinute => write!(f, "1m"),
            TimeInterval::FiveMinutes => write!(f, "5m"),
            TimeInterval::FifteenMinutes => write!(f, "15m"),
            TimeInterval::ThirtyMinutes => write!(f, "30m"),
            TimeInterval::OneHour => write!(f, "1h"),
            TimeInterval::FourHours => write!(f, "4h"),
            TimeInterval::OneDay => write!(f, "1d"),
            TimeInterval::Custom(s) => write!(f, "{}s", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn test_ohlc_creation() {
        let ohlc = OHLC::new(
            Decimal::from_f64(100.0).unwrap(),
            Decimal::from_f64(105.0).unwrap(),
            Decimal::from_f64(99.0).unwrap(),
            Decimal::from_f64(103.0).unwrap(),
            1000, 1234567890
        );
        assert_eq!(ohlc.open, Decimal::from_f64(100.0).unwrap());
        assert_eq!(ohlc.high, Decimal::from_f64(105.0).unwrap());
        assert_eq!(ohlc.low, Decimal::from_f64(99.0).unwrap());
        assert_eq!(ohlc.close, Decimal::from_f64(103.0).unwrap());
        assert_eq!(ohlc.volume.value(), 1000);
        assert_eq!(ohlc.timestamp, 1234567890);
    }

    #[test]
    fn test_ohlc_validation() {
        let valid_ohlc = OHLC::new(
            Decimal::from_f64(100.0).unwrap(),
            Decimal::from_f64(105.0).unwrap(),
            Decimal::from_f64(99.0).unwrap(),
            Decimal::from_f64(103.0).unwrap(),
            1000, 1234567890
        );
        assert!(valid_ohlc.is_valid());

        // Invalid: high < close
        let invalid_ohlc = OHLC::new(
            Decimal::from_f64(100.0).unwrap(),
            Decimal::from_f64(102.0).unwrap(),
            Decimal::from_f64(99.0).unwrap(),
            Decimal::from_f64(103.0).unwrap(),
            1000, 1234567890
        );
        assert!(!invalid_ohlc.is_valid());
    }

    #[test]
    fn test_ohlc_calculations() {
        let ohlc = OHLC::new(
            Decimal::from_f64(100.0).unwrap(),
            Decimal::from_f64(105.0).unwrap(),
            Decimal::from_f64(99.0).unwrap(),
            Decimal::from_f64(103.0).unwrap(),
            1000, 1234567890
        );
        assert_eq!(ohlc.range(), Decimal::from_f64(6.0).unwrap());
        assert_eq!(ohlc.body(), Decimal::from_f64(3.0).unwrap());
        assert!(ohlc.is_bullish());

        let bearish = OHLC::new(
            Decimal::from_f64(100.0).unwrap(),
            Decimal::from_f64(102.0).unwrap(),
            Decimal::from_f64(97.0).unwrap(),
            Decimal::from_f64(98.0).unwrap(),
            1000, 1234567890
        );
        assert!(!bearish.is_bullish());
    }

    #[test]
    fn test_tick_creation() {
        let tick = Tick::new(
            Decimal::from_f64(100.5).unwrap(),
            500, 1234567890
        );
        assert_eq!(tick.price, Decimal::from_f64(100.5).unwrap());
        assert_eq!(tick.volume.value(), 500);
        assert_eq!(tick.timestamp, 1234567890);
        assert!(tick.bid.is_none());
        assert!(tick.ask.is_none());
    }

    #[test]
    fn test_tick_with_spread() {
        let tick = Tick::with_spread(
            Decimal::from_f64(100.5).unwrap(),
            500, 1234567890,
            Decimal::from_f64(100.4).unwrap(),
            Decimal::from_f64(100.6).unwrap()
        );
        assert_eq!(tick.bid, Some(Decimal::from_f64(100.4).unwrap()));
        assert_eq!(tick.ask, Some(Decimal::from_f64(100.6).unwrap()));
        
        // Decimal precision is exact, no epsilon needed
        let spread = tick.spread().unwrap();
        assert_eq!(spread, Decimal::from_f64(0.2).unwrap());
    }

    #[test]
    fn test_time_interval() {
        assert_eq!(TimeInterval::OneMinute.seconds(), 60);
        assert_eq!(TimeInterval::FiveMinutes.seconds(), 300);
        assert_eq!(TimeInterval::OneHour.seconds(), 3600);
        assert_eq!(TimeInterval::OneDay.seconds(), 86400);
        assert_eq!(TimeInterval::Custom(120).seconds(), 120);

        assert_eq!(TimeInterval::OneMinute.millis(), 60000);
    }

    #[test]
    fn test_volume() {
        let vol = Volume::new(1500);
        assert_eq!(vol.value(), 1500);
        assert_eq!(vol.as_f64(), 1500.0);
        assert_eq!(format!("{}", vol), "1500");
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_ohlc_serialization() {
            let ohlc = OHLC::new(
                Decimal::from_f64(100.0).unwrap(),
                Decimal::from_f64(105.0).unwrap(),
                Decimal::from_f64(99.0).unwrap(),
                Decimal::from_f64(103.0).unwrap(),
                1000, 1234567890
            );
            
            // Serialize to JSON
            let json = serde_json::to_string(&ohlc).unwrap();
            
            // Deserialize back
            let deserialized: OHLC = serde_json::from_str(&json).unwrap();
            
            assert_eq!(ohlc, deserialized);
        }

        #[test]
        fn test_tick_serialization() {
            let tick = Tick::with_spread(
                Decimal::from_f64(100.5).unwrap(),
                500, 1234567890,
                Decimal::from_f64(100.4).unwrap(),
                Decimal::from_f64(100.6).unwrap()
            );
            
            let json = serde_json::to_string(&tick).unwrap();
            let deserialized: Tick = serde_json::from_str(&json).unwrap();
            
            assert_eq!(tick, deserialized);
        }

        #[test]
        fn test_volume_serialization() {
            let volume = Volume::new(1500);
            
            let json = serde_json::to_string(&volume).unwrap();
            assert_eq!(json, "1500"); // Should serialize as transparent
            
            let deserialized: Volume = serde_json::from_str(&json).unwrap();
            assert_eq!(volume, deserialized);
        }

        #[test]
        fn test_time_interval_serialization() {
            // Test standard intervals
            let interval = TimeInterval::OneMinute;
            let json = serde_json::to_string(&interval).unwrap();
            assert_eq!(json, r#""1m""#);
            
            let deserialized: TimeInterval = serde_json::from_str(&json).unwrap();
            assert_eq!(interval, deserialized);
            
            // Test custom interval
            let custom = TimeInterval::Custom(120);
            let json = serde_json::to_string(&custom).unwrap();
            let deserialized: TimeInterval = serde_json::from_str(&json).unwrap();
            assert_eq!(custom, deserialized);
        }
    }
}