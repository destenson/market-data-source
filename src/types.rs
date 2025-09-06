//! Core data types for market data representation.

use std::fmt;

/// Represents an OHLC (Open, High, Low, Close) candle.
///
/// This is the standard representation for aggregated price data over a time period.
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
    /// Total volume traded during the period
    pub volume: u64,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
}

impl OHLC {
    /// Creates a new OHLC candle.
    ///
    /// # Panics
    /// Panics if high < max(open, close) or low > min(open, close)
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: u64, timestamp: i64) -> Self {
        let candle = Self {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        };
        candle.validate();
        candle
    }

    /// Validates that OHLC values are consistent.
    fn validate(&self) {
        let max_body = self.open.max(self.close);
        let min_body = self.open.min(self.close);
        
        assert!(
            self.high >= max_body,
            "High ({:.1}) must be >= max(open, close) ({:.1})",
            self.high,
            max_body
        );
        
        assert!(
            self.low <= min_body,
            "Low ({:.1}) must be <= min(open, close) ({:.1})",
            self.low,
            min_body
        );
        
        assert!(
            self.high >= self.low,
            "High ({}) must be >= low ({})",
            self.high,
            self.low
        );
    }

    /// Returns the price range (high - low).
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Returns the body size (|close - open|).
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Returns true if this is a green/bullish candle (close > open).
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Returns true if this is a red/bearish candle (close < open).
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
}

impl fmt::Display for OHLC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OHLC(O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{} T:{})",
            self.open, self.high, self.low, self.close, self.volume, self.timestamp
        )
    }
}

/// Represents a single tick of market data.
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    /// Price at this tick
    pub price: f64,
    /// Volume at this tick
    pub volume: u64,
    /// Unix timestamp in milliseconds
    pub timestamp: i64,
    /// Optional bid price
    pub bid: Option<f64>,
    /// Optional ask price
    pub ask: Option<f64>,
}

impl Tick {
    /// Creates a new tick with just price and volume.
    pub fn new(price: f64, volume: u64, timestamp: i64) -> Self {
        Self {
            price,
            volume,
            timestamp,
            bid: None,
            ask: None,
        }
    }

    /// Creates a new tick with bid/ask spread.
    pub fn with_spread(price: f64, volume: u64, timestamp: i64, bid: f64, ask: f64) -> Self {
        Self {
            price,
            volume,
            timestamp,
            bid: Some(bid),
            ask: Some(ask),
        }
    }

    /// Returns the bid-ask spread if both are available.
    pub fn spread(&self) -> Option<f64> {
        match (self.bid, self.ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }
}

/// Represents volume data.
#[derive(Debug, Clone, PartialEq)]
pub struct Volume {
    /// Total volume
    pub total: u64,
    /// Buy volume
    pub buy: u64,
    /// Sell volume
    pub sell: u64,
}

impl Volume {
    /// Creates a new volume structure.
    pub fn new(total: u64, buy: u64, sell: u64) -> Self {
        assert_eq!(
            total,
            buy + sell,
            "Total volume must equal buy + sell volume"
        );
        Self { total, buy, sell }
    }

    /// Returns the buy/sell ratio.
    pub fn buy_sell_ratio(&self) -> f64 {
        if self.sell == 0 {
            f64::INFINITY
        } else {
            self.buy as f64 / self.sell as f64
        }
    }

    /// Returns the buy percentage (0.0 to 1.0).
    pub fn buy_percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.buy as f64 / self.total as f64
        }
    }
}

/// Time intervals for candle aggregation.
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
    /// Custom interval in milliseconds
    Custom(u64),
}

impl TimeInterval {
    /// Returns the interval duration in milliseconds.
    pub fn as_millis(&self) -> u64 {
        match self {
            Self::OneMinute => 60_000,
            Self::FiveMinutes => 300_000,
            Self::FifteenMinutes => 900_000,
            Self::ThirtyMinutes => 1_800_000,
            Self::OneHour => 3_600_000,
            Self::FourHours => 14_400_000,
            Self::OneDay => 86_400_000,
            Self::Custom(ms) => *ms,
        }
    }

    /// Returns the interval duration in seconds.
    pub fn as_secs(&self) -> u64 {
        self.as_millis() / 1000
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneMinute => write!(f, "1m"),
            Self::FiveMinutes => write!(f, "5m"),
            Self::FifteenMinutes => write!(f, "15m"),
            Self::ThirtyMinutes => write!(f, "30m"),
            Self::OneHour => write!(f, "1h"),
            Self::FourHours => write!(f, "4h"),
            Self::OneDay => write!(f, "1d"),
            Self::Custom(ms) => write!(f, "{}ms", ms),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ohlc_creation() {
        let ohlc = OHLC::new(100.0, 105.0, 99.0, 102.0, 1000, 1234567890);
        assert_eq!(ohlc.open, 100.0);
        assert_eq!(ohlc.high, 105.0);
        assert_eq!(ohlc.low, 99.0);
        assert_eq!(ohlc.close, 102.0);
        assert_eq!(ohlc.volume, 1000);
        assert_eq!(ohlc.timestamp, 1234567890);
    }

    #[test]
    #[should_panic(expected = "High (99.0) must be >= max(open, close) (102.0)")]
    fn test_ohlc_invalid_high() {
        OHLC::new(100.0, 99.0, 99.0, 102.0, 1000, 1234567890);
    }

    #[test]
    #[should_panic(expected = "Low (103.0) must be <= min(open, close) (100.0)")]
    fn test_ohlc_invalid_low() {
        OHLC::new(100.0, 105.0, 103.0, 102.0, 1000, 1234567890);
    }

    #[test]
    fn test_ohlc_analysis() {
        let bullish = OHLC::new(100.0, 105.0, 99.0, 104.0, 1000, 0);
        assert!(bullish.is_bullish());
        assert!(!bullish.is_bearish());
        assert_eq!(bullish.range(), 6.0);
        assert_eq!(bullish.body_size(), 4.0);

        let bearish = OHLC::new(100.0, 101.0, 95.0, 96.0, 1000, 0);
        assert!(!bearish.is_bullish());
        assert!(bearish.is_bearish());
        assert_eq!(bearish.range(), 6.0);
        assert_eq!(bearish.body_size(), 4.0);
    }

    #[test]
    fn test_tick_creation() {
        let tick = Tick::new(100.5, 500, 1234567890);
        assert_eq!(tick.price, 100.5);
        assert_eq!(tick.volume, 500);
        assert_eq!(tick.spread(), None);

        let tick_with_spread = Tick::with_spread(100.5, 500, 1234567890, 100.4, 100.6);
        let spread = tick_with_spread.spread().unwrap();
        assert!((spread - 0.2).abs() < 0.0001, "Spread {} not close to 0.2", spread);
    }

    #[test]
    fn test_volume() {
        let volume = Volume::new(1000, 600, 400);
        assert_eq!(volume.buy_percentage(), 0.6);
        assert_eq!(volume.buy_sell_ratio(), 1.5);
    }

    #[test]
    fn test_time_interval() {
        assert_eq!(TimeInterval::OneMinute.as_millis(), 60_000);
        assert_eq!(TimeInterval::OneHour.as_secs(), 3600);
        assert_eq!(TimeInterval::Custom(12345).as_millis(), 12345);
        assert_eq!(format!("{}", TimeInterval::FiveMinutes), "5m");
    }
}