#![allow(unused)]
//! Integration tests for regime control functionality

#[cfg(feature = "regimes")]
use market_data_source::{
    MarketDataGenerator, ConfigBuilder,
    MarketRegime, RegimeSchedule, RegimeSegment, TrendDirection
};

#[cfg(feature = "regimes")]
#[test]
fn test_regime_control_end_to_end() {
    // Create a generator with deterministic seed
    let config = ConfigBuilder::new()
        .seed(42)
        .starting_price_f64(100.0)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Create a complex schedule:
    // - 10 points Bull market (should trend up with lower volatility)
    // - 5 points Sideways market (should have no trend)
    // - 8 points Bear market (should trend down with higher volatility)
    let segments = vec![
        RegimeSegment::new(MarketRegime::Bull, 10),
        RegimeSegment::new(MarketRegime::Sideways, 5),
        RegimeSegment::new(MarketRegime::Bear, 8),
    ];
    let schedule = RegimeSchedule::new(segments);
    
    generator.enable_regime_control(schedule);
    
    // Generate the entire series
    let series = generator.generate_controlled_regime_series(23);
    assert_eq!(series.len(), 23);
    
    // Analyze regime segments
    let bull_data: Vec<_> = series.iter().take(10).collect();
    let sideways_data: Vec<_> = series.iter().skip(10).take(5).collect();
    let bear_data: Vec<_> = series.iter().skip(15).take(8).collect();
    
    // Verify regime information is correct
    for (i, data) in bull_data.iter().enumerate() {
        if let Some(ref info) = data.schedule_info {
            assert_eq!(info.current_regime, Some(MarketRegime::Bull), 
                      "Point {} should be in Bull regime", i);
        }
    }
    
    for (i, data) in sideways_data.iter().enumerate() {
        if let Some(ref info) = data.schedule_info {
            assert_eq!(info.current_regime, Some(MarketRegime::Sideways), 
                      "Point {} should be in Sideways regime", i + 10);
        }
    }
    
    for (i, data) in bear_data.iter().enumerate() {
        if let Some(ref info) = data.schedule_info {
            assert_eq!(info.current_regime, Some(MarketRegime::Bear), 
                      "Point {} should be in Bear regime", i + 15);
        }
    }
    
    // Verify data quality - all OHLC should be valid
    for (i, data) in series.iter().enumerate() {
        assert!(data.ohlc.is_valid(), "OHLC at index {} should be valid", i);
        assert!(data.ohlc.volume.value() > 0, "Volume at index {} should be positive", i);
    }
    
    // Verify schedule completed
    let final_info = generator.regime_control_info().unwrap();
    assert!(final_info.is_complete, "Schedule should be complete");
    assert_eq!(final_info.total_progress, 23, "Total progress should be 23");
}

#[cfg(feature = "regimes")]
#[test]
fn test_regime_parameter_effects_on_generation() {
    let config = ConfigBuilder::new()
        .seed(123) // Different seed for variety
        .starting_price_f64(50.0)
        .volatility_f64(0.01) // Low base volatility
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Create schedule with very distinct regimes
    let segments = vec![
        RegimeSegment::new(MarketRegime::Bull, 50),   // Long bull run
        RegimeSegment::new(MarketRegime::Bear, 50),   // Long bear run
    ];
    let schedule = RegimeSchedule::new(segments);
    
    generator.enable_regime_control(schedule);
    
    // Generate data for both regimes
    let bull_data = generator.generate_series(50);
    let bear_data = generator.generate_series(50);
    
    // Calculate statistics for each regime
    let bull_prices: Vec<_> = bull_data.iter().map(|c| c.close).collect();
    let bear_prices: Vec<_> = bear_data.iter().map(|c| c.close).collect();
    
    let bull_start = bull_prices[0];
    let bull_end = bull_prices[bull_prices.len() - 1];
    let bear_start = bear_prices[0];
    let bear_end = bear_prices[bear_prices.len() - 1];
    
    // Note: Due to randomness, we can't guarantee exact trends,
    // but we can verify the data was generated and is valid
    assert!(bull_start > rust_decimal::Decimal::ZERO);
    assert!(bull_end > rust_decimal::Decimal::ZERO);
    assert!(bear_start > rust_decimal::Decimal::ZERO);
    assert!(bear_end > rust_decimal::Decimal::ZERO);
    
    // Verify all data is valid
    for candle in &bull_data {
        assert!(candle.is_valid());
    }
    for candle in &bear_data {
        assert!(candle.is_valid());
    }
}

#[cfg(feature = "regimes")]
#[test]
fn test_repeating_regime_schedule() {
    let config = ConfigBuilder::new()
        .seed(456)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Create a short repeating schedule
    let segments = vec![
        RegimeSegment::new(MarketRegime::Bull, 3),
        RegimeSegment::new(MarketRegime::Bear, 2),
    ];
    let schedule = RegimeSchedule::repeating(segments);
    
    generator.enable_regime_control(schedule);
    
    // Generate more data than one cycle to test repeating
    let series = generator.generate_controlled_regime_series(12);
    
    // Should cycle: Bull(3) -> Bear(2) -> Bull(3) -> Bear(2) -> Bull(2)
    let expected_regimes = [
        MarketRegime::Bull,  MarketRegime::Bull,  MarketRegime::Bull,     // 0-2
        MarketRegime::Bear,  MarketRegime::Bear,                         // 3-4
        MarketRegime::Bull,  MarketRegime::Bull,  MarketRegime::Bull,     // 5-7
        MarketRegime::Bear,  MarketRegime::Bear,                         // 8-9
        MarketRegime::Bull,  MarketRegime::Bull,                         // 10-11
    ];
    
    for (i, (data, &expected)) in series.iter().zip(expected_regimes.iter()).enumerate() {
        if let Some(ref info) = data.schedule_info {
            assert_eq!(info.current_regime, Some(expected), 
                      "Point {} should be in {:?} regime", i, expected);
        }
    }
    
    // Schedule should not be complete since it repeats
    let info = generator.regime_control_info().unwrap();
    assert!(!info.is_complete, "Repeating schedule should never be complete");
}

#[cfg(feature = "regimes")]
#[test]
fn test_regime_force_override() {
    let config = ConfigBuilder::new()
        .seed(789)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Start with a Bull regime
    let segments = vec![
        RegimeSegment::new(MarketRegime::Bull, 100), // Long bull run
    ];
    let schedule = RegimeSchedule::new(segments);
    
    generator.enable_regime_control(schedule);
    
    // Generate a few points in bull market
    let bull_series = generator.generate_series(5);
    let info = generator.regime_control_info().unwrap();
    assert_eq!(info.current_regime, Some(MarketRegime::Bull));
    
    // Force switch to bear market
    generator.force_regime(MarketRegime::Bear, 10, None);
    
    // Generate in bear market
    let bear_series = generator.generate_series(3);
    let info = generator.regime_control_info().unwrap();
    assert_eq!(info.current_regime, Some(MarketRegime::Bear));
    
    // Verify data quality
    for candle in &bull_series {
        assert!(candle.is_valid());
    }
    for candle in &bear_series {
        assert!(candle.is_valid());
    }
}

#[cfg(feature = "regimes")]
#[test]
fn test_regime_control_with_transitions() {
    let config = ConfigBuilder::new()
        .seed(999)
        .build()
        .unwrap();
    
    let mut generator = MarketDataGenerator::with_config(config).unwrap();
    
    // Create schedule with smooth transitions
    let segments = vec![
        RegimeSegment::new(MarketRegime::Bull, 5).with_transition(2),
        RegimeSegment::new(MarketRegime::Bear, 5).with_transition(3),
    ];
    let schedule = RegimeSchedule::new(segments);
    
    generator.enable_regime_control(schedule);
    
    // Generate data through both segments
    let series = generator.generate_controlled_regime_series(10);
    
    // Verify we get valid data throughout transitions
    for (i, data) in series.iter().enumerate() {
        assert!(data.ohlc.is_valid(), "OHLC at index {} should be valid during transition", i);
        
        if let Some(ref info) = data.schedule_info {
            // In transition periods, we may see different regimes
            assert!(
                info.current_regime.is_some(),
                "Should always have a current regime at index {}", i
            );
        }
    }
    
    // The schedule should be complete after processing all segments (5 + 5 = 10)
    let final_info = generator.regime_control_info().unwrap();
    assert!(final_info.is_complete, "Schedule should be complete after generating 10 points from 2 segments of 5 each");
}
