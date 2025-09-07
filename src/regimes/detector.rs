//! Trait definition for regime detection strategies

use rust_decimal::Decimal;
use crate::types::OHLC;
use super::{MarketRegime, RegimeState, RegimeConfig};

/// Trait for implementing different regime detection strategies
pub trait RegimeDetector: Send + Sync {
    /// Detects the current market regime based on price data
    ///
    /// # Arguments
    /// * `data` - Historical OHLC data for analysis
    /// * `config` - Configuration parameters for detection
    ///
    /// # Returns
    /// The detected market regime with confidence level
    fn detect(&mut self, data: &[OHLC], config: &RegimeConfig) -> Option<RegimeState>;

    /// Updates the detector with a new data point
    ///
    /// # Arguments
    /// * `candle` - New OHLC candle to process
    /// * `config` - Configuration parameters for detection
    ///
    /// # Returns
    /// Updated regime state if detection criteria are met
    fn update(&mut self, candle: &OHLC, config: &RegimeConfig) -> Option<RegimeState>;

    /// Calculates the confidence level for a detected regime
    ///
    /// # Arguments
    /// * `regime` - The detected market regime
    /// * `data` - Historical data used for detection
    ///
    /// # Returns
    /// Confidence level between 0.0 and 1.0
    fn calculate_confidence(&self, regime: MarketRegime, data: &[OHLC]) -> Decimal;

    /// Resets the detector to its initial state
    fn reset(&mut self);

    /// Returns the name of the detection strategy
    fn name(&self) -> &str;

    /// Checks if the detector has enough data for analysis
    fn has_sufficient_data(&self, data_points: usize, config: &RegimeConfig) -> bool {
        data_points >= config.lookback_period
    }
}

/// Helper functions for regime detection
pub mod helpers {
    use rust_decimal::Decimal;
    use crate::types::OHLC;
    use super::MarketRegime;

    /// Calculates the simple return between two prices
    pub fn calculate_return(price_old: Decimal, price_new: Decimal) -> Decimal {
        if price_old == Decimal::ZERO {
            Decimal::ZERO
        } else {
            (price_new - price_old) / price_old
        }
    }

    /// Calculates returns for a series of OHLC data
    pub fn calculate_returns(data: &[OHLC]) -> Vec<Decimal> {
        if data.len() < 2 {
            return vec![];
        }

        data.windows(2)
            .map(|window| calculate_return(window[0].close, window[1].close))
            .collect()
    }

    /// Calculates the cumulative return over a period
    pub fn calculate_cumulative_return(data: &[OHLC]) -> Decimal {
        if data.len() < 2 {
            return Decimal::ZERO;
        }

        let first = data.first().unwrap();
        let last = data.last().unwrap();
        calculate_return(first.close, last.close)
    }

    /// Calculates the average return
    pub fn calculate_average_return(returns: &[Decimal]) -> Decimal {
        if returns.is_empty() {
            return Decimal::ZERO;
        }

        let sum: Decimal = returns.iter().sum();
        sum / Decimal::from(returns.len())
    }

    /// Calculates the standard deviation of returns
    pub fn calculate_volatility(returns: &[Decimal]) -> Decimal {
        if returns.len() < 2 {
            return Decimal::ZERO;
        }

        let mean = calculate_average_return(returns);
        let variance: Decimal = returns
            .iter()
            .map(|r| {
                let diff = r - mean;
                diff * diff
            })
            .sum::<Decimal>() / Decimal::from(returns.len() - 1);

        // Approximate square root calculation
        sqrt_approximation(variance)
    }

    /// Approximates square root using Newton's method
    pub fn sqrt_approximation(value: Decimal) -> Decimal {
        if value <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        let mut x = value;
        let mut last_x = Decimal::ZERO;
        let epsilon = Decimal::new(1, 6); // 0.000001

        // Newton's method iteration
        while (x - last_x).abs() > epsilon {
            last_x = x;
            x = (x + value / x) / Decimal::TWO;
        }

        x
    }

    /// Identifies trend direction based on moving averages
    pub fn identify_trend(data: &[OHLC], short_period: usize, long_period: usize) -> MarketRegime {
        if data.len() < long_period {
            return MarketRegime::Sideways;
        }

        let short_ma = calculate_sma(&data[data.len() - short_period..]);
        let long_ma = calculate_sma(&data[data.len() - long_period..]);

        let difference = (short_ma - long_ma) / long_ma;
        let threshold = Decimal::new(1, 2); // 0.01 = 1%

        if difference > threshold {
            MarketRegime::Bull
        } else if difference < -threshold {
            MarketRegime::Bear
        } else {
            MarketRegime::Sideways
        }
    }

    /// Calculates Simple Moving Average
    pub fn calculate_sma(data: &[OHLC]) -> Decimal {
        if data.is_empty() {
            return Decimal::ZERO;
        }

        let sum: Decimal = data.iter().map(|candle| candle.close).sum();
        sum / Decimal::from(data.len())
    }

    /// Calculates Exponential Moving Average
    pub fn calculate_ema(data: &[OHLC], period: usize) -> Decimal {
        if data.is_empty() {
            return Decimal::ZERO;
        }

        let alpha = Decimal::TWO / (Decimal::from(period + 1));
        let mut ema = data[0].close;

        for candle in &data[1..] {
            ema = alpha * candle.close + (Decimal::ONE - alpha) * ema;
        }

        ema
    }

    /// Counts the number of higher highs and higher lows (uptrend indicators)
    pub fn count_higher_highs_lows(data: &[OHLC]) -> (usize, usize) {
        if data.len() < 2 {
            return (0, 0);
        }

        let mut higher_highs = 0;
        let mut higher_lows = 0;

        for window in data.windows(2) {
            if window[1].high > window[0].high {
                higher_highs += 1;
            }
            if window[1].low > window[0].low {
                higher_lows += 1;
            }
        }

        (higher_highs, higher_lows)
    }

    /// Counts the number of lower highs and lower lows (downtrend indicators)
    pub fn count_lower_highs_lows(data: &[OHLC]) -> (usize, usize) {
        if data.len() < 2 {
            return (0, 0);
        }

        let mut lower_highs = 0;
        let mut lower_lows = 0;

        for window in data.windows(2) {
            if window[1].high < window[0].high {
                lower_highs += 1;
            }
            if window[1].low < window[0].low {
                lower_lows += 1;
            }
        }

        (lower_highs, lower_lows)
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::*;
    use super::MarketRegime;
    use crate::types::OHLC;
    use rust_decimal::Decimal;

    fn create_test_candle(close: i64) -> OHLC {
        OHLC::new(
            Decimal::from(close),
            Decimal::from(close + 1),
            Decimal::from(close - 1),
            Decimal::from(close),
            1000,
            1000000,
        )
    }

    #[test]
    fn test_calculate_return() {
        let return_val = calculate_return(Decimal::from(100), Decimal::from(110));
        assert_eq!(return_val, Decimal::new(1, 1)); // 0.1 = 10%

        let return_val = calculate_return(Decimal::from(100), Decimal::from(90));
        assert_eq!(return_val, Decimal::new(-1, 1)); // -0.1 = -10%
    }

    #[test]
    fn test_calculate_returns() {
        let data = vec![
            create_test_candle(100),
            create_test_candle(110),
            create_test_candle(105),
        ];

        let returns = calculate_returns(&data);
        assert_eq!(returns.len(), 2);
        assert_eq!(returns[0], Decimal::new(1, 1)); // 10%
    }

    #[test]
    fn test_sqrt_approximation() {
        let result = sqrt_approximation(Decimal::from(4));
        assert!((result - Decimal::TWO).abs() < Decimal::new(1, 3)); // Close to 2

        let result = sqrt_approximation(Decimal::from(9));
        assert!((result - Decimal::from(3)).abs() < Decimal::new(1, 3)); // Close to 3
    }

    #[test]
    fn test_calculate_sma() {
        let data = vec![
            create_test_candle(100),
            create_test_candle(110),
            create_test_candle(120),
        ];

        let sma = calculate_sma(&data);
        assert_eq!(sma, Decimal::from(110)); // (100 + 110 + 120) / 3
    }

    #[test]
    fn test_identify_trend() {
        // Create uptrending data
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(create_test_candle(100 + i * 2));
        }

        let trend = identify_trend(&data, 5, 10);
        assert_eq!(trend, MarketRegime::Bull);
    }
}