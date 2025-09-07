#[cfg(test)]
pub mod test_helpers {
    use rust_decimal::Decimal;
    
    /// Helper function to convert f64 to Decimal for test data
    pub fn d(val: f64) -> Decimal {
        Decimal::from_f64_retain(val).unwrap_or_else(|| {
            panic!("Failed to convert {} to Decimal", val)
        })
    }
    
    /// Create test OHLC data with Decimal values
    pub fn create_test_ohlc(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: u64,
        timestamp: i64,
    ) -> crate::OHLC {
        crate::OHLC::new(d(open), d(high), d(low), d(close), volume, timestamp)
    }
    
    /// Create test Tick data with Decimal values
    pub fn create_test_tick(price: f64, volume: u64, timestamp: i64) -> crate::Tick {
        crate::Tick::new(d(price), volume, timestamp)
    }
    
    /// Create test Tick with spread data
    pub fn create_test_tick_with_spread(
        price: f64,
        volume: u64,
        timestamp: i64,
        bid: f64,
        ask: f64,
    ) -> crate::Tick {
        crate::Tick::with_spread(d(price), volume, timestamp, d(bid), d(ask))
    }
}