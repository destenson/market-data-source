//! Market regime detection and analysis module

use rust_decimal::Decimal;
use std::collections::VecDeque;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod controller;
pub mod detector;
pub mod statistics;
pub mod volatility;

pub use controller::{RegimeController, RegimeSchedule, RegimeSegment, ScheduleInfo, TransitionState};
pub use detector::RegimeDetector;
pub use statistics::RollingStatistics;
pub use volatility::VolatilityRegimeDetector;

/// Represents different market regime states
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketRegime {
    /// Bull market - persistent upward trend
    Bull,
    /// Bear market - persistent downward trend
    Bear,
    /// Sideways/ranging market - no clear directional trend
    Sideways,
    /// Market with normal volatility characteristics
    Normal {
        /// Mean price over the period
        mean: f64,
        /// Standard deviation of price
        std_dev: f64,
        /// Optional bias term for drift
        bias: Option<f64>,
    }
}

impl MarketRegime {
    /// Returns a human-readable description of the regime
    pub fn description(&self) -> &'static str {
        match self {
            MarketRegime::Bull => "Bull market - upward trend",
            MarketRegime::Bear => "Bear market - downward trend",
            MarketRegime::Sideways => "Sideways market - ranging",
            MarketRegime::Normal { .. } => "Normal market - volatility-based regime",
        }
    }

    /// Returns the typical volatility multiplier for this regime
    pub fn volatility_factor(&self) -> Decimal {
        match self {
            MarketRegime::Bull => Decimal::new(8, 1),      // 0.8 - lower volatility
            MarketRegime::Bear => Decimal::new(12, 1),     // 1.2 - higher volatility
            MarketRegime::Sideways => Decimal::ONE,        // 1.0 - normal volatility
            MarketRegime::Normal { std_dev, .. } => {
                // Use the std_dev parameter to determine volatility factor
                Decimal::try_from(*std_dev).unwrap_or(Decimal::ONE)
            }
        }
    }

    /// Returns the typical drift factor for this regime
    pub fn drift_factor(&self) -> Decimal {
        match self {
            MarketRegime::Bull => Decimal::new(5, 3),      // 0.005 - positive drift
            MarketRegime::Bear => Decimal::new(-5, 3),     // -0.005 - negative drift
            MarketRegime::Sideways => Decimal::ZERO,       // 0 - no drift
            MarketRegime::Normal { bias, .. } => {
                // Use bias if available, otherwise zero drift
                let drift = bias.unwrap_or(0.0);
                Decimal::try_from(drift).unwrap_or(Decimal::ZERO)
            }
        }
    }
}

/// Holds the current regime state and metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegimeState {
    /// Current market regime
    pub current_regime: MarketRegime,
    /// Confidence level of the regime detection (0.0 to 1.0)
    pub confidence: Decimal,
    /// Number of periods in current regime
    pub duration: usize,
    /// Timestamp when the regime started
    pub start_timestamp: i64,
    /// Price level when regime started
    pub start_price: Decimal,
}

impl RegimeState {
    /// Creates a new regime state
    pub fn new(regime: MarketRegime, confidence: Decimal, timestamp: i64, price: Decimal) -> Self {
        Self {
            current_regime: regime,
            confidence,
            duration: 1,
            start_timestamp: timestamp,
            start_price: price,
        }
    }

    /// Updates the duration of the current regime
    pub fn increment_duration(&mut self) {
        self.duration += 1;
    }

    /// Checks if a regime transition should occur
    pub fn should_transition(&self, new_regime: MarketRegime, new_confidence: Decimal) -> bool {
        // Transition if new regime is different with high confidence
        // or if confidence in current regime has dropped significantly
        new_regime != self.current_regime && new_confidence > Decimal::new(6, 1) // 0.6
            || self.confidence < Decimal::new(3, 1) // 0.3
    }

    /// Transitions to a new regime
    pub fn transition(&mut self, new_regime: MarketRegime, confidence: Decimal, timestamp: i64, price: Decimal) {
        self.current_regime = new_regime;
        self.confidence = confidence;
        self.duration = 1;
        self.start_timestamp = timestamp;
        self.start_price = price;
    }
}

/// Configuration for regime detection
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegimeConfig {
    /// Lookback period for regime detection
    pub lookback_period: usize,
    /// Threshold for bull market detection (e.g., 0.02 for 2% return)
    pub bull_threshold: Decimal,
    /// Threshold for bear market detection (e.g., -0.02 for -2% return)
    pub bear_threshold: Decimal,
    /// Minimum confidence required for regime change
    pub min_confidence: Decimal,
    /// Enable volatility-based regime detection
    pub use_volatility: bool,
    /// Window size for rolling statistics
    pub window_size: usize,
}

impl Default for RegimeConfig {
    fn default() -> Self {
        Self {
            lookback_period: 20,
            bull_threshold: Decimal::new(2, 2),    // 0.02
            bear_threshold: Decimal::new(-2, 2),   // -0.02
            min_confidence: Decimal::new(6, 1),    // 0.6
            use_volatility: true,
            window_size: 20,
        }
    }
}

/// Tracks regime transitions and provides analytics
#[derive(Debug, Clone)]
pub struct RegimeTracker {
    /// History of regime states
    pub history: VecDeque<RegimeState>,
    /// Maximum history size
    max_history: usize,
    /// Transition count
    pub transitions: usize,
}

impl RegimeTracker {
    /// Creates a new regime tracker
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history),
            max_history,
            transitions: 0,
        }
    }

    /// Records a new regime state
    pub fn record(&mut self, state: RegimeState) {
        if let Some(last) = self.history.back() {
            if last.current_regime != state.current_regime {
                self.transitions += 1;
            }
        }
        
        self.history.push_back(state);
        
        // Maintain max history size
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Gets the current regime state
    pub fn current(&self) -> Option<&RegimeState> {
        self.history.back()
    }

    /// Calculates the average regime duration
    pub fn average_duration(&self) -> Decimal {
        if self.history.is_empty() {
            return Decimal::ZERO;
        }

        let total_duration: usize = self.history.iter().map(|s| s.duration).sum();
        Decimal::from(total_duration) / Decimal::from(self.history.len())
    }

    /// Gets regime distribution (percentage of time in each regime)
    pub fn regime_distribution(&self) -> (Decimal, Decimal, Decimal) {
        if self.history.is_empty() {
            return (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
        }

        let mut bull_periods = 0;
        let mut bear_periods = 0;
        let mut sideways_periods = 0;

        for state in &self.history {
            match state.current_regime {
                MarketRegime::Bull => bull_periods += state.duration,
                MarketRegime::Bear => bear_periods += state.duration,
                MarketRegime::Sideways => sideways_periods += state.duration,
                MarketRegime::Normal { .. } => sideways_periods += state.duration, // Treat Normal as sideways for distribution
            }
        }

        let total = bull_periods + bear_periods + sideways_periods;
        if total == 0 {
            return (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
        }

        let total_dec = Decimal::from(total);
        (
            Decimal::from(bull_periods) / total_dec,
            Decimal::from(bear_periods) / total_dec,
            Decimal::from(sideways_periods) / total_dec,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_regime_properties() {
        assert_eq!(MarketRegime::Bull.volatility_factor(), Decimal::new(8, 1));
        assert_eq!(MarketRegime::Bear.volatility_factor(), Decimal::new(12, 1));
        assert_eq!(MarketRegime::Sideways.volatility_factor(), Decimal::ONE);

        assert_eq!(MarketRegime::Bull.drift_factor(), Decimal::new(5, 3));
        assert_eq!(MarketRegime::Bear.drift_factor(), Decimal::new(-5, 3));
        assert_eq!(MarketRegime::Sideways.drift_factor(), Decimal::ZERO);
    }

    #[test]
    fn test_regime_state() {
        let mut state = RegimeState::new(
            MarketRegime::Bull,
            Decimal::new(8, 1),
            1000,
            Decimal::new(100, 0),
        );

        assert_eq!(state.current_regime, MarketRegime::Bull);
        assert_eq!(state.duration, 1);

        state.increment_duration();
        assert_eq!(state.duration, 2);

        // Test transition logic
        assert!(state.should_transition(MarketRegime::Bear, Decimal::new(7, 1)));
        assert!(!state.should_transition(MarketRegime::Bear, Decimal::new(5, 1)));
    }

    #[test]
    fn test_regime_tracker() {
        let mut tracker = RegimeTracker::new(10);
        
        let state1 = RegimeState::new(MarketRegime::Bull, Decimal::new(8, 1), 1000, Decimal::new(100, 0));
        let state2 = RegimeState::new(MarketRegime::Bear, Decimal::new(7, 1), 2000, Decimal::new(95, 0));
        
        tracker.record(state1.clone());
        assert_eq!(tracker.transitions, 0);
        
        tracker.record(state2);
        assert_eq!(tracker.transitions, 1);
        
        tracker.record(state1);
        assert_eq!(tracker.transitions, 2);
    }

    #[test]
    fn test_regime_config_default() {
        let config = RegimeConfig::default();
        assert_eq!(config.lookback_period, 20);
        assert_eq!(config.bull_threshold, Decimal::new(2, 2));
        assert_eq!(config.bear_threshold, Decimal::new(-2, 2));
        assert_eq!(config.min_confidence, Decimal::new(6, 1));
        assert!(config.use_volatility);
    }
}
