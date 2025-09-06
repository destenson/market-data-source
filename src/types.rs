//! Core data types for market data representation

use std::fmt;

/// Represents an OHLC (Open, High, Low, Close) candle
#[derive(Debug, Clone, PartialEq)]
pub struct OHLC {
    /// Opening price of the period
    pub open: f64,
    /// Highest price during the period
    pub high: f64,
    /// Lowest price during the period
    pub low: f64,
    /// Closing price of the period
    pub close: f64,
    /// Volume traded during the period
    pub volume: Volume,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
}

impl OHLC {
    /// Creates a new OHLC candle
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: u64, timestamp: i64) -> Self {
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
        self.open > 0.0 && self.high > 0.0 && 
        self.low > 0.0 && self.close > 0.0
    }

    /// Returns the price range (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Returns the body size (|close - open|)
    pub fn body(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Returns true if this is a green/bullish candle
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
}

/// Represents a single tick of market data
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    /// Price of the tick
    pub price: f64,
    /// Volume of the tick
    pub volume: Volume,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
    /// Optional bid price
    pub bid: Option<f64>,
    /// Optional ask price
    pub ask: Option<f64>,
}

impl Tick {
    /// Creates a new tick with just price and volume
    pub fn new(price: f64, volume: u64, timestamp: i64) -> Self {
        Self {
            price,
            volume: Volume::new(volume),
            timestamp,
            bid: None,
            ask: None,
        }
    }

    /// Creates a new tick with bid/ask spread
    pub fn with_spread(price: f64, volume: u64, timestamp: i64, bid: f64, ask: f64) -> Self {
        Self {
            price,
            volume: Volume::new(volume),
            timestamp,
            bid: Some(bid),
            ask: Some(ask),
        }
    }

    /// Returns the spread if bid and ask are available
    pub fn spread(&self) -> Option<f64> {
        match (self.bid, self.ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }
}

/// Represents volume in the market
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Volume {
    value: u64,
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
pub enum TimeInterval {
    /// One minute
    OneMinute,
    /// Five minutes
    FiveMinutes,
    /// Fifteen minutes
    FifteenMinutes,
    /// Thirty minutes
    ThirtyMinutes,
    /// One hour
    OneHour,
    /// Four hours
    FourHours,
    /// One day
    OneDay,
    /// Custom interval in seconds
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

    #[test]
    fn test_ohlc_creation() {
        let ohlc = OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1234567890);
        assert_eq!(ohlc.open, 100.0);
        assert_eq!(ohlc.high, 105.0);
        assert_eq!(ohlc.low, 99.0);
        assert_eq!(ohlc.close, 103.0);
        assert_eq!(ohlc.volume.value(), 1000);
        assert_eq!(ohlc.timestamp, 1234567890);
    }

    #[test]
    fn test_ohlc_validation() {
        let valid_ohlc = OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1234567890);
        assert!(valid_ohlc.is_valid());

        // Invalid: high < close
        let invalid_ohlc = OHLC::new(100.0, 102.0, 99.0, 103.0, 1000, 1234567890);
        assert!(!invalid_ohlc.is_valid());
    }

    #[test]
    fn test_ohlc_calculations() {
        let ohlc = OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1234567890);
        assert_eq!(ohlc.range(), 6.0);
        assert_eq!(ohlc.body(), 3.0);
        assert!(ohlc.is_bullish());

        let bearish = OHLC::new(100.0, 102.0, 97.0, 98.0, 1000, 1234567890);
        assert!(!bearish.is_bullish());
    }

    #[test]
    fn test_tick_creation() {
        let tick = Tick::new(100.5, 500, 1234567890);
        assert_eq!(tick.price, 100.5);
        assert_eq!(tick.volume.value(), 500);
        assert_eq!(tick.timestamp, 1234567890);
        assert!(tick.bid.is_none());
        assert!(tick.ask.is_none());
    }

    #[test]
    fn test_tick_with_spread() {
        let tick = Tick::with_spread(100.5, 500, 1234567890, 100.4, 100.6);
        assert_eq!(tick.bid, Some(100.4));
        assert_eq!(tick.ask, Some(100.6));
        
        // Use epsilon comparison for floating point
        let spread = tick.spread().unwrap();
        assert!((spread - 0.2).abs() < 1e-10);
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
}