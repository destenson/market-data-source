//! Rolling window statistics calculator for regime detection

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::VecDeque;
use crate::types::OHLC;

/// Calculates rolling statistics over a sliding window
pub struct RollingStatistics {
    /// Window size for calculations
    window_size: usize,
    /// Buffer for price data
    price_buffer: VecDeque<Decimal>,
    /// Buffer for returns
    return_buffer: VecDeque<Decimal>,
    /// Running sum of prices
    price_sum: Decimal,
    /// Running sum of returns
    return_sum: Decimal,
    /// Running sum of squared returns
    return_sum_squared: Decimal,
}

impl RollingStatistics {
    /// Creates a new rolling statistics calculator
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            price_buffer: VecDeque::with_capacity(window_size),
            return_buffer: VecDeque::with_capacity(window_size),
            price_sum: Decimal::ZERO,
            return_sum: Decimal::ZERO,
            return_sum_squared: Decimal::ZERO,
        }
    }

    /// Updates statistics with a new price
    pub fn update(&mut self, price: Decimal) {
        // Calculate return if we have previous price
        if let Some(&last_price) = self.price_buffer.back() {
            let return_val = if last_price != Decimal::ZERO {
                (price - last_price) / last_price
            } else {
                Decimal::ZERO
            };

            // Update return buffer and sums
            self.return_buffer.push_back(return_val);
            self.return_sum += return_val;
            self.return_sum_squared += return_val * return_val;

            // Remove old return if window is full
            if self.return_buffer.len() > self.window_size {
                if let Some(old_return) = self.return_buffer.pop_front() {
                    self.return_sum -= old_return;
                    self.return_sum_squared -= old_return * old_return;
                }
            }
        }

        // Update price buffer and sum
        self.price_buffer.push_back(price);
        self.price_sum += price;

        // Remove old price if window is full
        if self.price_buffer.len() > self.window_size {
            if let Some(old_price) = self.price_buffer.pop_front() {
                self.price_sum -= old_price;
            }
        }
    }

    /// Updates with an OHLC candle
    pub fn update_with_candle(&mut self, candle: &OHLC) {
        self.update(candle.close);
    }

    /// Gets the current mean price
    pub fn mean_price(&self) -> Decimal {
        if self.price_buffer.is_empty() {
            Decimal::ZERO
        } else {
            self.price_sum / Decimal::from(self.price_buffer.len())
        }
    }

    /// Gets the current mean return
    pub fn mean_return(&self) -> Decimal {
        if self.return_buffer.is_empty() {
            Decimal::ZERO
        } else {
            self.return_sum / Decimal::from(self.return_buffer.len())
        }
    }

    /// Gets the current standard deviation of returns (volatility)
    pub fn std_dev(&self) -> Decimal {
        if self.return_buffer.len() < 2 {
            return Decimal::ZERO;
        }

        let n = Decimal::from(self.return_buffer.len());
        let mean = self.mean_return();
        let variance = (self.return_sum_squared / n) - (mean * mean);

        // Approximate square root
        Self::sqrt_approximation(variance.abs())
    }

    /// Gets the current volatility (annualized)
    pub fn volatility(&self, periods_per_year: usize) -> Decimal {
        let daily_vol = self.std_dev();
        daily_vol * Self::sqrt_approximation(Decimal::from(periods_per_year))
    }

    /// Gets the Sharpe ratio (assuming risk-free rate of 0)
    pub fn sharpe_ratio(&self, periods_per_year: usize) -> Decimal {
        let vol = self.volatility(periods_per_year);
        if vol == Decimal::ZERO {
            Decimal::ZERO
        } else {
            let annualized_return = self.mean_return() * Decimal::from(periods_per_year);
            annualized_return / vol
        }
    }

    /// Gets the maximum drawdown in the window
    pub fn max_drawdown(&self) -> Decimal {
        if self.price_buffer.len() < 2 {
            return Decimal::ZERO;
        }

        let mut max_price = Decimal::ZERO;
        let mut max_dd = Decimal::ZERO;

        for &price in &self.price_buffer {
            if price > max_price {
                max_price = price;
            }
            if max_price > Decimal::ZERO {
                let dd = (max_price - price) / max_price;
                if dd > max_dd {
                    max_dd = dd;
                }
            }
        }

        max_dd
    }

    /// Gets the current momentum (price change over window)
    pub fn momentum(&self) -> Decimal {
        if self.price_buffer.len() < 2 {
            return Decimal::ZERO;
        }

        let first = self.price_buffer.front().unwrap();
        let last = self.price_buffer.back().unwrap();

        if *first != Decimal::ZERO {
            (last - first) / first
        } else {
            Decimal::ZERO
        }
    }

    /// Gets skewness of returns
    pub fn skewness(&self) -> Decimal {
        if self.return_buffer.len() < 3 {
            return Decimal::ZERO;
        }

        let mean = self.mean_return();
        let std_dev = self.std_dev();

        if std_dev == Decimal::ZERO {
            return Decimal::ZERO;
        }

        let n = Decimal::from(self.return_buffer.len());
        let mut sum_cubed = Decimal::ZERO;

        for &ret in &self.return_buffer {
            let diff = ret - mean;
            sum_cubed += diff * diff * diff;
        }

        let std_cubed = std_dev * std_dev * std_dev;
        (sum_cubed / n) / std_cubed
    }

    /// Gets kurtosis of returns
    pub fn kurtosis(&self) -> Decimal {
        if self.return_buffer.len() < 4 {
            return Decimal::ZERO;
        }

        let mean = self.mean_return();
        let variance = self.variance();

        if variance == Decimal::ZERO {
            return Decimal::ZERO;
        }

        let n = Decimal::from(self.return_buffer.len());
        let mut sum_fourth = Decimal::ZERO;

        for &ret in &self.return_buffer {
            let diff = ret - mean;
            sum_fourth += diff * diff * diff * diff;
        }

        let variance_squared = variance * variance;
        ((sum_fourth / n) / variance_squared) - Decimal::from(3)
    }

    /// Gets variance of returns
    pub fn variance(&self) -> Decimal {
        let std_dev = self.std_dev();
        std_dev * std_dev
    }

    /// Checks if statistics are ready (sufficient data)
    pub fn is_ready(&self) -> bool {
        self.return_buffer.len() >= self.window_size / 2
    }

    /// Resets all statistics
    pub fn reset(&mut self) {
        self.price_buffer.clear();
        self.return_buffer.clear();
        self.price_sum = Decimal::ZERO;
        self.return_sum = Decimal::ZERO;
        self.return_sum_squared = Decimal::ZERO;
    }

    /// Gets the current window size
    pub fn window_size(&self) -> usize {
        self.window_size
    }

    /// Gets the number of data points
    pub fn data_points(&self) -> usize {
        self.price_buffer.len()
    }

    /// Approximates square root using Newton's method
    fn sqrt_approximation(value: Decimal) -> Decimal {
        if value <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        let mut x = value;
        let mut last_x = Decimal::ZERO;
        let epsilon = Decimal::new(1, 6); // 0.000001

        let max_iterations = 20;
        let mut iterations = 0;

        while (x - last_x).abs() > epsilon && iterations < max_iterations {
            last_x = x;
            x = (x + value / x) / Decimal::TWO;
            iterations += 1;
        }

        x
    }
}

/// Provides return distribution analysis
pub struct ReturnDistribution {
    returns: Vec<Decimal>,
}

impl ReturnDistribution {
    /// Creates a new return distribution analyzer
    pub fn new() -> Self {
        Self {
            returns: Vec::new(),
        }
    }

    /// Adds returns from OHLC data
    pub fn add_from_ohlc(&mut self, data: &[OHLC]) {
        if data.len() < 2 {
            return;
        }

        for window in data.windows(2) {
            let return_val = if window[0].close != Decimal::ZERO {
                (window[1].close - window[0].close) / window[0].close
            } else {
                Decimal::ZERO
            };
            self.returns.push(return_val);
        }
    }

    /// Gets percentile of returns
    pub fn percentile(&self, p: Decimal) -> Decimal {
        if self.returns.is_empty() {
            return Decimal::ZERO;
        }

        let mut sorted = self.returns.clone();
        sorted.sort();

        let index = ((p * Decimal::from(sorted.len() - 1)).round()).to_usize().unwrap_or(0);
        sorted[index.min(sorted.len() - 1)]
    }

    /// Gets Value at Risk (VaR) at given confidence level
    pub fn var(&self, confidence: Decimal) -> Decimal {
        self.percentile(Decimal::ONE - confidence)
    }

    /// Gets Conditional Value at Risk (CVaR)
    pub fn cvar(&self, confidence: Decimal) -> Decimal {
        let var_threshold = self.var(confidence);
        let tail_returns: Vec<Decimal> = self.returns
            .iter()
            .filter(|&&r| r <= var_threshold)
            .copied()
            .collect();

        if tail_returns.is_empty() {
            var_threshold
        } else {
            tail_returns.iter().sum::<Decimal>() / Decimal::from(tail_returns.len())
        }
    }

    /// Clears all returns
    pub fn clear(&mut self) {
        self.returns.clear();
    }

    /// Gets the number of returns
    pub fn len(&self) -> usize {
        self.returns.len()
    }

    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.returns.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_statistics_basic() {
        let mut stats = RollingStatistics::new(5);

        // Add some prices
        for i in 1..=5 {
            stats.update(Decimal::from(100 + i));
        }

        assert_eq!(stats.mean_price(), Decimal::from(103)); // (101+102+103+104+105)/5
        assert!(stats.is_ready());
    }

    #[test]
    fn test_rolling_statistics_momentum() {
        let mut stats = RollingStatistics::new(5);

        stats.update(Decimal::from(100));
        stats.update(Decimal::from(102));
        stats.update(Decimal::from(104));
        stats.update(Decimal::from(106));
        stats.update(Decimal::from(110));

        let momentum = stats.momentum();
        assert_eq!(momentum, Decimal::new(1, 1)); // 10% increase
    }

    #[test]
    fn test_rolling_statistics_volatility() {
        let mut stats = RollingStatistics::new(10);

        // Add volatile prices
        for i in 0..10 {
            let price = if i % 2 == 0 {
                Decimal::from(100)
            } else {
                Decimal::from(105)
            };
            stats.update(price);
        }

        let vol = stats.std_dev();
        assert!(vol > Decimal::ZERO);
    }

    #[test]
    fn test_rolling_statistics_drawdown() {
        let mut stats = RollingStatistics::new(5);

        stats.update(Decimal::from(100));
        stats.update(Decimal::from(110));
        stats.update(Decimal::from(105));
        stats.update(Decimal::from(95));
        stats.update(Decimal::from(100));

        let dd = stats.max_drawdown();
        // Max was 110, min after was 95, so DD = (110-95)/110 ≈ 0.136
        assert!(dd > Decimal::new(13, 2) && dd < Decimal::new(14, 2));
    }

    #[test]
    fn test_return_distribution() {
        let mut dist = ReturnDistribution::new();

        let data = vec![
            OHLC::new(Decimal::from(100), Decimal::from(102), Decimal::from(98), Decimal::from(100), 1000, 1000),
            OHLC::new(Decimal::from(100), Decimal::from(103), Decimal::from(99), Decimal::from(102), 1000, 2000),
            OHLC::new(Decimal::from(102), Decimal::from(105), Decimal::from(101), Decimal::from(104), 1000, 3000),
        ];

        dist.add_from_ohlc(&data);
        assert_eq!(dist.len(), 2); // 2 returns from 3 prices

        let median = dist.percentile(Decimal::new(5, 1)); // 50th percentile
        assert!(median >= Decimal::ZERO);
    }

    #[test]
    fn test_sqrt_approximation() {
        let result = RollingStatistics::sqrt_approximation(Decimal::from(4));
        assert!((result - Decimal::TWO).abs() < Decimal::new(1, 3));

        let result = RollingStatistics::sqrt_approximation(Decimal::from(9));
        assert!((result - Decimal::from(3)).abs() < Decimal::new(1, 3));

        let result = RollingStatistics::sqrt_approximation(Decimal::from(16));
        assert!((result - Decimal::from(4)).abs() < Decimal::new(1, 3));
    }
}