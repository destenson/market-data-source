//! Regime control system for deterministic market regime management

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::config::{GeneratorConfig, TrendDirection};
use crate::regimes::MarketRegime;
use std::collections::VecDeque;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A scheduled segment of market regime with specific duration and parameters
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegimeSegment {
    /// The market regime for this segment
    pub regime: MarketRegime,
    /// Duration in number of data points
    pub duration: usize,
    /// Generator configuration for this regime
    pub config: GeneratorConfig,
    /// Optional transition duration for smooth parameter changes
    pub transition_duration: Option<usize>,
}

impl RegimeSegment {
    /// Creates a new regime segment with default configuration for the regime
    pub fn new(regime: MarketRegime, duration: usize) -> Self {
        let config = Self::default_config_for_regime(regime);
        Self {
            regime,
            duration,
            config,
            transition_duration: None,
        }
    }

    /// Creates a new regime segment with custom configuration
    pub fn with_config(regime: MarketRegime, duration: usize, config: GeneratorConfig) -> Self {
        Self {
            regime,
            duration,
            config,
            transition_duration: None,
        }
    }

    /// Sets the transition duration for smooth parameter changes
    pub fn with_transition(mut self, transition_duration: usize) -> Self {
        self.transition_duration = Some(transition_duration);
        self
    }

    /// Returns default configuration for a given market regime
    fn default_config_for_regime(regime: MarketRegime) -> GeneratorConfig {
        let mut config = GeneratorConfig::default();
        
        match regime {
            MarketRegime::Bull => {
                config.trend_direction = TrendDirection::Bullish;
                config.trend_strength = Decimal::new(5, 3); // 0.5% per period
                config.volatility = Decimal::new(15, 3); // 1.5%
            },
            MarketRegime::Bear => {
                config.trend_direction = TrendDirection::Bearish;
                config.trend_strength = Decimal::new(7, 3); // 0.7% per period (bear markets often sharper)
                config.volatility = Decimal::new(25, 3); // 2.5% (higher volatility in bear markets)
            },
            MarketRegime::Sideways => {
                config.trend_direction = TrendDirection::Sideways;
                config.trend_strength = Decimal::ZERO;
                config.volatility = Decimal::new(10, 3); // 1.0%
            },
            MarketRegime::Normal { std_dev, bias, .. } => {
                // Use the parameters from the Normal regime
                config.trend_direction = TrendDirection::Sideways; // Default to sideways
                config.trend_strength = Decimal::try_from(bias.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
                config.volatility = Decimal::try_from(std_dev).unwrap_or(Decimal::new(15, 3));
            },
        }
        
        config
    }
}

/// Manages a sequence of regime segments
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegimeSchedule {
    /// Queue of regime segments
    segments: VecDeque<RegimeSegment>,
    /// Points processed in current segment
    current_segment_progress: usize,
    /// Total points processed across all segments
    total_progress: usize,
    /// Whether the schedule should repeat when complete
    repeat: bool,
    /// Original schedule for repeating
    original_segments: Vec<RegimeSegment>,
}

impl RegimeSchedule {
    /// Creates a new regime schedule
    pub fn new(segments: Vec<RegimeSegment>) -> Self {
        let original_segments = segments.clone();
        Self {
            segments: VecDeque::from(segments),
            current_segment_progress: 0,
            total_progress: 0,
            repeat: false,
            original_segments,
        }
    }

    /// Creates a repeating regime schedule
    pub fn repeating(segments: Vec<RegimeSegment>) -> Self {
        let mut schedule = Self::new(segments);
        schedule.repeat = true;
        schedule
    }

    /// Gets the current regime segment
    pub fn current_segment(&self) -> Option<&RegimeSegment> {
        self.segments.front()
    }

    /// Advances to the next data point and returns current segment
    pub fn advance(&mut self) -> Option<&RegimeSegment> {
        self.current_segment_progress += 1;
        self.total_progress += 1;

        // Check if current segment is complete
        if let Some(current) = self.segments.front() {
            if self.current_segment_progress >= current.duration {
                // Move to next segment
                self.segments.pop_front();
                self.current_segment_progress = 0;

                // If schedule is empty and should repeat, reload original segments
                if self.segments.is_empty() && self.repeat {
                    self.segments = VecDeque::from(self.original_segments.clone());
                }
            }
        }

        self.segments.front()
    }

    /// Gets progress within current segment (0.0 to 1.0)
    pub fn current_segment_progress(&self) -> f64 {
        if let Some(current) = self.segments.front() {
            if current.duration == 0 {
                return 1.0;
            }
            self.current_segment_progress as f64 / current.duration as f64
        } else {
            1.0
        }
    }

    /// Gets total progress across all segments
    pub fn total_progress(&self) -> usize {
        self.total_progress
    }

    /// Checks if the schedule is complete
    pub fn is_complete(&self) -> bool {
        self.segments.is_empty() && !self.repeat
    }

    /// Resets the schedule to the beginning
    pub fn reset(&mut self) {
        self.segments = VecDeque::from(self.original_segments.clone());
        self.current_segment_progress = 0;
        self.total_progress = 0;
    }

    /// Adds a segment to the end of the schedule
    pub fn add_segment(&mut self, segment: RegimeSegment) {
        self.segments.push_back(segment.clone());
        self.original_segments.push(segment);
    }

    /// Gets remaining segments in the schedule
    pub fn remaining_segments(&self) -> Vec<&RegimeSegment> {
        self.segments.iter().collect()
    }

    /// Gets total duration of the schedule
    pub fn total_duration(&self) -> usize {
        self.original_segments.iter().map(|s| s.duration).sum()
    }
}

/// State information for parameter transitions between regimes
#[derive(Debug, Clone)]
pub struct TransitionState {
    /// Starting configuration
    pub from_config: GeneratorConfig,
    /// Target configuration
    pub to_config: GeneratorConfig,
    /// Transition progress (0.0 to 1.0)
    pub progress: f64,
    /// Total duration of transition
    pub duration: usize,
    /// Current step in transition
    pub current_step: usize,
}

impl TransitionState {
    /// Creates a new transition state
    pub fn new(from_config: GeneratorConfig, to_config: GeneratorConfig, duration: usize) -> Self {
        Self {
            from_config,
            to_config,
            progress: 0.0,
            duration,
            current_step: 0,
        }
    }

    /// Advances the transition by one step
    pub fn advance(&mut self) -> bool {
        if self.current_step < self.duration {
            self.current_step += 1;
            self.progress = self.current_step as f64 / self.duration as f64;
            true
        } else {
            false
        }
    }

    /// Gets the interpolated configuration at current progress
    pub fn interpolated_config(&self) -> GeneratorConfig {
        let mut config = self.from_config.clone();
        
        // Interpolate trend strength
        let from_strength = self.from_config.trend_strength;
        let to_strength = self.to_config.trend_strength;
        config.trend_strength = from_strength + (to_strength - from_strength) * Decimal::from_f64(self.progress).unwrap_or(Decimal::ZERO);
        
        // Interpolate volatility
        let from_vol = self.from_config.volatility;
        let to_vol = self.to_config.volatility;
        config.volatility = from_vol + (to_vol - from_vol) * Decimal::from_f64(self.progress).unwrap_or(Decimal::ZERO);
        
        // Trend direction switches at 50% progress
        if self.progress >= 0.5 {
            config.trend_direction = self.to_config.trend_direction;
        }
        
        config
    }

    /// Checks if transition is complete
    pub fn is_complete(&self) -> bool {
        self.current_step >= self.duration
    }
}

/// Main regime controller that manages regime schedules and parameter transitions
#[derive(Debug)]
pub struct RegimeController {
    /// Current regime schedule
    schedule: RegimeSchedule,
    /// Current generator configuration
    current_config: GeneratorConfig,
    /// Active transition state
    transition: Option<TransitionState>,
    /// Base configuration to merge regime-specific settings with
    base_config: GeneratorConfig,
}

impl RegimeController {
    /// Creates a new regime controller with a schedule
    pub fn new(schedule: RegimeSchedule, base_config: GeneratorConfig) -> Self {
        let current_config = if let Some(segment) = schedule.current_segment() {
            Self::merge_configs(&base_config, &segment.config)
        } else {
            base_config.clone()
        };

        Self {
            schedule,
            current_config,
            transition: None,
            base_config,
        }
    }

    /// Gets the current generator configuration
    pub fn current_config(&self) -> &GeneratorConfig {
        &self.current_config
    }

    /// Gets the current regime
    pub fn current_regime(&self) -> Option<MarketRegime> {
        self.schedule.current_segment().map(|s| s.regime)
    }

    /// Advances to the next data point and updates configuration
    pub fn advance(&mut self) -> bool {
        // Advance transition if active
        if let Some(ref mut transition) = self.transition {
            if transition.advance() {
                self.current_config = transition.interpolated_config();
                if transition.is_complete() {
                    self.transition = None;
                }
            }
        }

        // Check for regime changes
        let previous_regime = self.current_regime();
        let current_segment = self.schedule.advance();
        let new_regime = current_segment.map(|s| s.regime);

        // Handle regime change
        if previous_regime != new_regime {
            if let Some(segment) = current_segment {
                let new_config = Self::merge_configs(&self.base_config, &segment.config);
                
                // Start transition if specified
                if let Some(transition_duration) = segment.transition_duration {
                    if transition_duration > 0 {
                        self.transition = Some(TransitionState::new(
                            self.current_config.clone(),
                            new_config,
                            transition_duration,
                        ));
                    } else {
                        self.current_config = new_config;
                    }
                } else {
                    self.current_config = new_config;
                }
            }
        }

        !self.schedule.is_complete()
    }

    /// Merges base configuration with regime-specific configuration
    fn merge_configs(base: &GeneratorConfig, regime_specific: &GeneratorConfig) -> GeneratorConfig {
        let mut config = base.clone();
        
        // Override regime-specific parameters
        config.trend_direction = regime_specific.trend_direction;
        config.trend_strength = regime_specific.trend_strength;
        config.volatility = regime_specific.volatility;
        
        config
    }

    /// Gets current schedule information
    pub fn schedule_info(&self) -> ScheduleInfo {
        ScheduleInfo {
            current_regime: self.current_regime(),
            current_segment_progress: self.schedule.current_segment_progress(),
            total_progress: self.schedule.total_progress(),
            is_complete: self.schedule.is_complete(),
            remaining_segments: self.schedule.remaining_segments().len(),
            in_transition: self.transition.is_some(),
        }
    }

    /// Replaces the current schedule
    pub fn set_schedule(&mut self, schedule: RegimeSchedule) {
        self.schedule = schedule;
        if let Some(segment) = self.schedule.current_segment() {
            self.current_config = Self::merge_configs(&self.base_config, &segment.config);
        }
        self.transition = None;
    }

    /// Adds a segment to the current schedule
    pub fn add_segment(&mut self, segment: RegimeSegment) {
        self.schedule.add_segment(segment);
    }

    /// Resets the schedule to the beginning
    pub fn reset(&mut self) {
        self.schedule.reset();
        self.transition = None;
        if let Some(segment) = self.schedule.current_segment() {
            self.current_config = Self::merge_configs(&self.base_config, &segment.config);
        }
    }

    /// Force immediate regime change (bypassing schedule)
    pub fn force_regime(&mut self, regime: MarketRegime, duration: usize, transition_duration: Option<usize>) {
        let segment = RegimeSegment::new(regime, duration)
            .with_transition(transition_duration.unwrap_or(0));
        
        let new_schedule = RegimeSchedule::new(vec![segment]);
        self.set_schedule(new_schedule);
    }
}

/// Information about the current schedule state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScheduleInfo {
    /// Current regime
    pub current_regime: Option<MarketRegime>,
    /// Progress within current segment (0.0 to 1.0)
    pub current_segment_progress: f64,
    /// Total data points processed
    pub total_progress: usize,
    /// Whether schedule is complete
    pub is_complete: bool,
    /// Number of remaining segments
    pub remaining_segments: usize,
    /// Whether currently in a parameter transition
    pub in_transition: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regime_segment_creation() {
        let segment = RegimeSegment::new(MarketRegime::Bull, 100);
        assert_eq!(segment.regime, MarketRegime::Bull);
        assert_eq!(segment.duration, 100);
        assert_eq!(segment.config.trend_direction, TrendDirection::Bullish);
    }

    #[test]
    fn test_regime_schedule_basic() {
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 50),
            RegimeSegment::new(MarketRegime::Bear, 30),
        ];
        
        let mut schedule = RegimeSchedule::new(segments);
        
        // Should start with Bull regime
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bull);
        
        // Advance 49 times - should still be in Bull
        for _ in 0..49 {
            schedule.advance();
        }
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bull);
        
        // Advance once more - should switch to Bear
        schedule.advance();
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bear);
    }

    #[test]
    fn test_regime_schedule_completion() {
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 2),
        ];
        
        let mut schedule = RegimeSchedule::new(segments);
        
        assert!(!schedule.is_complete());
        schedule.advance(); // 1
        assert!(!schedule.is_complete());
        schedule.advance(); // 2
        assert!(schedule.is_complete());
    }

    #[test]
    fn test_regime_schedule_repeating() {
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 1),
            RegimeSegment::new(MarketRegime::Bear, 1),
        ];
        
        let mut schedule = RegimeSchedule::repeating(segments);
        
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bull);
        schedule.advance(); // Move to Bear
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bear);
        schedule.advance(); // Should cycle back to Bull
        assert_eq!(schedule.current_segment().unwrap().regime, MarketRegime::Bull);
        assert!(!schedule.is_complete()); // Never completes when repeating
    }

    #[test]
    fn test_transition_state() {
        let from_config = GeneratorConfig::default();
        let mut to_config = GeneratorConfig::default();
        to_config.volatility = Decimal::new(2, 1); // 0.2
        
        let mut transition = TransitionState::new(from_config, to_config, 4);
        
        assert_eq!(transition.progress, 0.0);
        
        transition.advance();
        assert_eq!(transition.progress, 0.25);
        
        let interpolated = transition.interpolated_config();
        // Should be 25% of the way from 0.02 to 0.2
        let expected = Decimal::new(2, 2) + (Decimal::new(2, 1) - Decimal::new(2, 2)) * Decimal::new(25, 2);
        assert_eq!(interpolated.volatility, expected);
    }

    #[test]
    fn test_regime_controller_basic() {
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 3),
            RegimeSegment::new(MarketRegime::Bear, 2),
        ];
        
        let schedule = RegimeSchedule::new(segments);
        let base_config = GeneratorConfig::default();
        let mut controller = RegimeController::new(schedule, base_config);
        
        assert_eq!(controller.current_regime(), Some(MarketRegime::Bull));
        assert_eq!(controller.current_config().trend_direction, TrendDirection::Bullish);
        
        // Advance through Bull regime
        for _ in 0..3 {
            controller.advance();
        }
        
        assert_eq!(controller.current_regime(), Some(MarketRegime::Bear));
        assert_eq!(controller.current_config().trend_direction, TrendDirection::Bearish);
    }
}