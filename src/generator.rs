#![allow(unused)]
//! Main market data generator

use crate::algorithms::RandomWalkGenerator;
use crate::config::GeneratorConfig;
use crate::types::{Tick, OHLC};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "regimes")]
use crate::regimes::{
    RegimeConfig, RegimeController, RegimeDetector, RegimeSchedule, RegimeState, RegimeTracker,
    ScheduleInfo, VolatilityRegimeDetector,
};

/// Main market data generator
pub struct MarketDataGenerator {
    /// Random number generator
    rng: StdRng,
    /// Configuration
    config: GeneratorConfig,
    /// Price generator algorithm
    price_generator: RandomWalkGenerator,
    /// Current timestamp in milliseconds
    current_timestamp: i64,
    /// Regime detector for market state analysis
    #[cfg(feature = "regimes")]
    regime_detector: Option<Box<dyn RegimeDetector>>,
    /// Regime tracker for historical analysis
    #[cfg(feature = "regimes")]
    regime_tracker: Option<RegimeTracker>,
    /// Historical data buffer for regime detection
    #[cfg(feature = "regimes")]
    data_buffer: Vec<OHLC>,
    /// Regime controller for deterministic regime management
    #[cfg(feature = "regimes")]
    regime_controller: Option<RegimeController>,
}

impl MarketDataGenerator {
    /// Creates a new generator with default configuration
    pub fn new() -> Self {
        Self::with_config(GeneratorConfig::default()).expect("Default config should be valid")
    }

    /// Creates a new generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Result<Self, String> {
        // Validate configuration
        config
            .validate()
            .map_err(|e| format!("Invalid configuration: {e}"))?;

        // Create RNG with seed if provided
        let rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        // Create price generator
        let price_generator = RandomWalkGenerator::new(config.clone())?;

        // Get current timestamp
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get system time: {e}"))?
            .as_millis() as i64;

        Ok(Self {
            rng,
            config: config.clone(),
            price_generator,
            current_timestamp,
            #[cfg(feature = "regimes")]
            regime_detector: None,
            #[cfg(feature = "regimes")]
            regime_tracker: None,
            #[cfg(feature = "regimes")]
            data_buffer: Vec::with_capacity(100),
            #[cfg(feature = "regimes")]
            regime_controller: None,
        })
    }

    /// Generates a single OHLC
    pub fn generate_ohlc(&mut self) -> OHLC {
        // Update regime controller if enabled
        #[cfg(feature = "regimes")]
        self.update_regime_controller();

        // Generate OHLC prices (using 10 ticks per candle for realism)
        let (open, high, low, close) = self.price_generator.generate_ohlc(&mut self.rng, 10);

        // Generate volume
        let volume = self.price_generator.generate_volume(&mut self.rng);

        // Get timestamp
        let timestamp = self.current_timestamp;

        // Advance timestamp for next candle
        self.current_timestamp += self.config.time_interval.millis() as i64;

        let candle = OHLC::new(open, high, low, close, volume, timestamp);

        // Update regime detection if enabled
        #[cfg(feature = "regimes")]
        self.update_regime_detection(&candle);

        candle
    }

    /// Generates a single OHLC candle
    #[deprecated(since = "0.2.0", note = "Use generate_ohlc() instead")]
    pub fn generate_candle(&mut self) -> OHLC {
        self.generate_ohlc()
    }

    /// Generates a series of OHLC candles
    pub fn generate_series(&mut self, count: usize) -> Vec<OHLC> {
        let mut candles = Vec::with_capacity(count);
        for _ in 0..count {
            candles.push(self.generate_ohlc());
        }
        candles
    }

    /// Generates a single tick
    pub fn generate_tick(&mut self) -> Tick {
        let price = self.price_generator.next_price(&mut self.rng);
        let volume = self.price_generator.generate_volume(&mut self.rng);
        let timestamp = self.current_timestamp;

        // Advance timestamp by 1 second for ticks
        self.current_timestamp += 1000;

        // Optionally generate bid/ask spread
        let spread = Decimal::from_f64(0.001).unwrap(); // 0.1% spread
        let half_spread = price * spread / Decimal::from(2);

        Tick::with_spread(
            price,
            volume,
            timestamp,
            price - half_spread,
            price + half_spread,
        )
    }

    /// Generates a series of ticks
    pub fn generate_ticks(&mut self, count: usize) -> Vec<Tick> {
        let mut ticks = Vec::with_capacity(count);
        for _ in 0..count {
            ticks.push(self.generate_tick());
        }
        ticks
    }

    /// Resets the generator to initial state
    pub fn reset(&mut self) {
        self.price_generator.reset();
        self.current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        // Reset RNG with seed if provided
        if let Some(seed) = self.config.seed {
            self.rng = StdRng::seed_from_u64(seed);
        }
    }

    /// Sets a specific starting timestamp
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.current_timestamp = timestamp;
    }

    /// Gets the current configuration
    pub fn config(&self) -> &GeneratorConfig {
        &self.config
    }

    /// Updates the configuration
    pub fn set_config(&mut self, config: GeneratorConfig) -> Result<(), String> {
        config
            .validate()
            .map_err(|e| format!("Invalid configuration: {e}"))?;

        // Update config
        self.config = config.clone();

        // Recreate price generator with new config
        self.price_generator = RandomWalkGenerator::new(config)?;

        // Update RNG if seed changed
        if let Some(seed) = self.config.seed {
            self.rng = StdRng::seed_from_u64(seed);
        }

        Ok(())
    }

    /// Generate OHLC data and export to CSV file
    #[cfg(feature = "csv_export")]
    pub fn generate_to_csv_ohlc<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.generate_series(count);
        crate::export::to_csv_ohlc(&data, path)?;
        Ok(())
    }

    /// Generate tick data and export to CSV file
    #[cfg(feature = "csv_export")]
    pub fn generate_to_csv_ticks<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.generate_ticks(count);
        crate::export::to_csv_ticks(&data, path)?;
        Ok(())
    }

    /// Stream generate OHLC data directly to CSV file (memory efficient for large datasets)
    #[cfg(feature = "csv_export")]
    pub fn stream_generate_to_csv_ohlc<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::export::csv::CsvExporter;

        let exporter = CsvExporter::default();

        // Create an iterator that generates candles on-the-fly
        let iter = (0..count).map(|_| self.generate_ohlc());

        Ok(exporter.stream_ohlc(iter, path)?)
    }

    /// Stream generate tick data directly to CSV file (memory efficient for large datasets)
    #[cfg(feature = "csv_export")]
    pub fn stream_generate_to_csv_ticks<P: AsRef<std::path::Path>>(
        &mut self,
        count: usize,
        path: P,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::export::csv::CsvExporter;

        let exporter = CsvExporter::default();

        // Create an iterator that generates ticks on-the-fly
        let iter = (0..count).map(|_| self.generate_tick());

        Ok(exporter.stream_ticks(iter, path)?)
    }
}

// Regime detection methods (enabled with "regimes" feature)
#[cfg(feature = "regimes")]
impl MarketDataGenerator {
    // Rule-based detector removed due to missing implementation

    /// Enables regime detection with volatility-based detector
    pub fn enable_volatility_regime_detection(&mut self, window_size: usize) {
        self.regime_detector = Some(Box::new(VolatilityRegimeDetector::new(window_size)));
        self.regime_tracker = Some(RegimeTracker::new(1000));
    }

    /// Disables regime detection
    pub fn disable_regime_detection(&mut self) {
        self.regime_detector = None;
        self.regime_tracker = None;
        self.data_buffer.clear();
    }

    /// Gets the current regime state
    pub fn current_regime(&self) -> Option<&RegimeState> {
        self.regime_tracker.as_ref()?.current()
    }

    /// Gets regime detection analytics
    pub fn regime_analytics(&self) -> Option<RegimeAnalytics> {
        let tracker = self.regime_tracker.as_ref()?;
        Some(RegimeAnalytics {
            current_regime: tracker.current().cloned(),
            transitions: tracker.transitions,
            average_duration: tracker.average_duration(),
            regime_distribution: tracker.regime_distribution(),
        })
    }

    /// Updates regime detection with a new candle
    fn update_regime_detection(&mut self, candle: &OHLC) {
        // Add to buffer
        self.data_buffer.push(candle.clone());

        // Maintain buffer size (keep last 200 candles)
        while self.data_buffer.len() > 200 {
            self.data_buffer.remove(0);
        }

        // Update regime detection if detector is enabled
        if let Some(ref mut detector) = self.regime_detector {
            let config = RegimeConfig::default();
            if let Some(state) = detector.update(candle, &config) {
                if let Some(ref mut tracker) = self.regime_tracker {
                    tracker.record(state);
                }
            }
        }
    }

    /// Generates regime-aware OHLC series with market transitions
    pub fn generate_regime_aware_series(
        &mut self,
        count: usize,
        _regime_config: RegimeConfig,
    ) -> Vec<RegimeOHLC> {
        let mut series = Vec::with_capacity(count);

        for _ in 0..count {
            let candle = self.generate_ohlc();
            let current_regime = self.current_regime().cloned();
            series.push(RegimeOHLC {
                ohlc: candle,
                regime_state: current_regime,
            });
        }

        series
    }

    /// Generates OHLC series with controlled regime schedule
    pub fn generate_controlled_regime_series(&mut self, count: usize) -> Vec<ControlledRegimeOHLC> {
        let mut series = Vec::with_capacity(count);

        for _ in 0..count {
            let schedule_info = self.regime_control_info();
            let candle = self.generate_ohlc();

            series.push(ControlledRegimeOHLC {
                ohlc: candle,
                schedule_info: schedule_info.clone(),
            });
        }

        series
    }

    /// Gets the data buffer for analysis
    pub fn data_buffer(&self) -> &[OHLC] {
        &self.data_buffer
    }

    /// Detects regime on historical data buffer
    pub fn detect_regime_on_buffer(&mut self) -> Option<RegimeState> {
        if let Some(ref mut detector) = self.regime_detector {
            let config = RegimeConfig::default();
            detector.detect(&self.data_buffer, &config)
        } else {
            None
        }
    }

    /// Enables regime control with a given schedule
    pub fn enable_regime_control(&mut self, schedule: RegimeSchedule) {
        let base_config = self.config.clone();
        self.regime_controller = Some(RegimeController::new(schedule, base_config));
    }

    /// Disables regime control
    pub fn disable_regime_control(&mut self) {
        self.regime_controller = None;
    }

    /// Gets current regime control information
    pub fn regime_control_info(&self) -> Option<ScheduleInfo> {
        self.regime_controller.as_ref().map(|c| c.schedule_info())
    }

    /// Forces an immediate regime change (overriding current schedule)
    pub fn force_regime(
        &mut self,
        regime: crate::regimes::MarketRegime,
        duration: usize,
        transition_duration: Option<usize>,
    ) {
        if let Some(ref mut controller) = self.regime_controller {
            controller.force_regime(regime, duration, transition_duration);
        }
    }

    /// Adds a regime segment to the current schedule
    pub fn add_regime_segment(&mut self, segment: crate::regimes::RegimeSegment) {
        if let Some(ref mut controller) = self.regime_controller {
            controller.add_segment(segment);
        }
    }

    /// Resets the regime schedule to the beginning
    pub fn reset_regime_schedule(&mut self) {
        if let Some(ref mut controller) = self.regime_controller {
            controller.reset();
        }
    }

    /// Updates regime controller and applies configuration changes
    fn update_regime_controller(&mut self) {
        if let Some(ref mut controller) = self.regime_controller {
            controller.advance();

            // Update the generator configuration with the regime-controlled config
            let new_config = controller.current_config().clone();
            if let Ok(new_price_generator) = RandomWalkGenerator::new(new_config.clone()) {
                self.config = new_config;
                self.price_generator = new_price_generator;
            }
        }
    }
}

/// OHLC data with regime information
#[cfg(feature = "regimes")]
#[derive(Debug, Clone)]
pub struct RegimeOHLC {
    /// OHLC candle data
    pub ohlc: OHLC,
    /// Associated regime state (if detected)
    pub regime_state: Option<RegimeState>,
}

/// OHLC data with controlled regime schedule information
#[cfg(feature = "regimes")]
#[derive(Debug, Clone)]
pub struct ControlledRegimeOHLC {
    /// OHLC candle data
    pub ohlc: OHLC,
    /// Current schedule information from regime controller
    pub schedule_info: Option<ScheduleInfo>,
}

/// Regime detection analytics
#[cfg(feature = "regimes")]
#[derive(Debug, Clone)]
pub struct RegimeAnalytics {
    /// Current regime state
    pub current_regime: Option<RegimeState>,
    /// Total number of regime transitions
    pub transitions: usize,
    /// Average regime duration
    pub average_duration: Decimal,
    /// Time spent in each regime (bull, bear, sideways)
    pub regime_distribution: (Decimal, Decimal, Decimal),
}

impl Default for MarketDataGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigBuilder;
    use crate::TrendDirection;

    #[test]
    fn test_generator_creation() {
        let generator = MarketDataGenerator::new();
        assert_eq!(
            generator.config().starting_price,
            Decimal::from_f64(100.0).unwrap()
        );
    }

    #[test]
    fn test_generator_with_config() {
        let config = ConfigBuilder::new()
            .starting_price_f64(50.0)
            .volatility_f64(0.03)
            .seed(42)
            .build()
            .unwrap();

        let generator = MarketDataGenerator::with_config(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_candle_generation() {
        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let candle = generator.generate_ohlc();

        assert!(candle.is_valid());
        assert!(candle.volume.value() > 0);
    }

    #[test]
    fn test_series_generation() {
        let config = ConfigBuilder::new()
            .seed(42)
            .num_points(10)
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();
        let series = generator.generate_series(10);

        assert_eq!(series.len(), 10);

        // Check timestamps are increasing
        for i in 1..series.len() {
            assert!(series[i].timestamp > series[i - 1].timestamp);
        }
    }

    #[test]
    fn test_tick_generation() {
        let mut generator = MarketDataGenerator::new();
        let tick = generator.generate_tick();

        assert!(tick.price > Decimal::ZERO);
        assert!(tick.volume.value() > 0);
        assert!(tick.bid.is_some());
        assert!(tick.ask.is_some());

        if let (Some(bid), Some(ask)) = (tick.bid, tick.ask) {
            assert!(ask > bid); // Spread should be positive
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut gen1 = MarketDataGenerator::with_config(config.clone()).unwrap();
        let mut gen2 = MarketDataGenerator::with_config(config).unwrap();

        let candles1 = gen1.generate_series(5);
        let candles2 = gen2.generate_series(5);

        // With same seed, should generate same data
        for (c1, c2) in candles1.iter().zip(candles2.iter()) {
            assert_eq!(c1.open, c2.open);
            assert_eq!(c1.close, c2.close);
        }
    }

    #[test]
    fn test_reset() {
        let config = ConfigBuilder::new()
            .seed(42)
            .starting_price_f64(100.0)
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        let candle1 = generator.generate_ohlc();
        generator.reset();
        let candle2 = generator.generate_ohlc();

        // After reset with same seed, should generate same values
        assert_eq!(candle1.open, candle2.open);
    }

    #[cfg(feature = "regimes")]
    #[test]
    fn test_regime_control_basic() {
        use crate::config::TrendDirection;
        use crate::regimes::{MarketRegime, RegimeSchedule, RegimeSegment};

        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        // Create a simple schedule: 5 points Bull, 5 points Bear
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 5),
            RegimeSegment::new(MarketRegime::Bear, 5),
        ];
        let schedule = RegimeSchedule::new(segments);

        generator.enable_regime_control(schedule);

        // Generate first batch - should be bull market
        let bull_candles = generator.generate_series(5);
        let info = generator.regime_control_info().unwrap();
        assert_eq!(info.current_regime, Some(MarketRegime::Bear)); // Should have switched after 5

        // Generate second batch - should be bear market
        let bear_candles = generator.generate_series(5);
        let info = generator.regime_control_info().unwrap();
        assert!(info.is_complete); // Should be complete after 10 total

        // Verify we have proper OHLC data
        assert_eq!(bull_candles.len(), 5);
        assert_eq!(bear_candles.len(), 5);

        for candle in &bull_candles {
            assert!(candle.is_valid());
        }
        for candle in &bear_candles {
            assert!(candle.is_valid());
        }
    }

    #[cfg(feature = "regimes")]
    #[test]
    fn test_regime_control_parameter_changes() {
        use crate::regimes::{MarketRegime, RegimeSchedule, RegimeSegment};

        let config = ConfigBuilder::new()
            .seed(42)
            .trend_f64(TrendDirection::Sideways, 0.001) // Very small base trend
            .build()
            .unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        // Create schedule with different regimes
        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 3),
            RegimeSegment::new(MarketRegime::Sideways, 3),
        ];
        let schedule = RegimeSchedule::new(segments);

        generator.enable_regime_control(schedule);

        // Generate during bull period
        let config1 = generator.config().clone();
        generator.generate_ohlc();
        generator.generate_ohlc();

        // Should still be bull
        assert_eq!(generator.config().trend_direction, TrendDirection::Bullish);

        // Generate one more to trigger regime change
        generator.generate_ohlc();

        // Should now be sideways
        let config2 = generator.config().clone();
        assert_eq!(config2.trend_direction, TrendDirection::Sideways);
        assert_ne!(config1.trend_strength, config2.trend_strength); // Should be different
    }

    #[cfg(feature = "regimes")]
    #[test]
    fn test_regime_control_force_regime() {
        use crate::regimes::{MarketRegime, RegimeSchedule, RegimeSegment};

        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        // Start with a schedule
        let segments = vec![RegimeSegment::new(MarketRegime::Bull, 10)];
        let schedule = RegimeSchedule::new(segments);
        generator.enable_regime_control(schedule);

        // Verify initial regime
        let info = generator.regime_control_info().unwrap();
        assert_eq!(info.current_regime, Some(MarketRegime::Bull));

        // Force a different regime
        generator.force_regime(MarketRegime::Bear, 5, None);

        // Verify regime changed
        let info = generator.regime_control_info().unwrap();
        assert_eq!(info.current_regime, Some(MarketRegime::Bear));
    }

    #[cfg(feature = "regimes")]
    #[test]
    fn test_controlled_regime_series() {
        use crate::regimes::{MarketRegime, RegimeSchedule, RegimeSegment};

        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 3),
            RegimeSegment::new(MarketRegime::Bear, 2),
        ];
        let schedule = RegimeSchedule::new(segments);
        generator.enable_regime_control(schedule);

        // Generate controlled series
        let series = generator.generate_controlled_regime_series(5);

        assert_eq!(series.len(), 5);

        // First 3 should be bull market
        for i in 0..3 {
            if let Some(ref info) = series[i].schedule_info {
                assert_eq!(info.current_regime, Some(MarketRegime::Bull));
            }
        }

        // Last 2 should be bear market
        for i in 3..5 {
            if let Some(ref info) = series[i].schedule_info {
                assert_eq!(info.current_regime, Some(MarketRegime::Bear));
            }
        }
    }

    #[cfg(feature = "regimes")]
    #[test]
    fn test_regime_schedule_reset() {
        use crate::regimes::{MarketRegime, RegimeSchedule, RegimeSegment};

        let config = ConfigBuilder::new().seed(42).build().unwrap();

        let mut generator = MarketDataGenerator::with_config(config).unwrap();

        let segments = vec![
            RegimeSegment::new(MarketRegime::Bull, 2),
            RegimeSegment::new(MarketRegime::Bear, 2),
        ];
        let schedule = RegimeSchedule::new(segments);
        generator.enable_regime_control(schedule);

        // Generate all data
        generator.generate_series(4);
        let info = generator.regime_control_info().unwrap();
        assert!(info.is_complete);

        // Reset schedule
        generator.reset_regime_schedule();
        let info = generator.regime_control_info().unwrap();
        assert!(!info.is_complete);
        assert_eq!(info.current_regime, Some(MarketRegime::Bull)); // Should be back to start
    }
}
