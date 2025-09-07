#!/usr/bin/env python3
"""
Unit tests for market_data_source Python bindings.
"""

import unittest
import os
import tempfile
import json
from pathlib import Path

# Import the module (will be available after building)
try:
    import market_data_source as mds
except ImportError:
    import sys
    print("Warning: market_data_source not installed. Run 'maturin develop' first.")
    sys.exit(1)


class TestMarketDataGenerator(unittest.TestCase):
    """Test cases for MarketDataGenerator class."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.generator = mds.MarketDataGenerator(
            initial_price=100.0,
            volatility=0.02,
            seed=42
        )
        self.temp_dir = tempfile.mkdtemp()
    
    def tearDown(self):
        """Clean up test fixtures."""
        # Clean up temporary files
        import shutil
        if os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
    
    def test_generator_creation(self):
        """Test generator creation with various parameters."""
        # Default creation
        gen1 = mds.MarketDataGenerator()
        self.assertIsNotNone(gen1)
        
        # With parameters
        gen2 = mds.MarketDataGenerator(
            initial_price=200.0,
            volatility=0.03,
            trend=0.001,
            min_price=150.0,
            max_price=250.0,
            volume_base=5000,  # Must be integer
            volume_volatility=0.2,
            interval="5m",
            seed=123
        )
        self.assertIsNotNone(gen2)
        
        # Check config
        config = gen2.config
        self.assertAlmostEqual(config.initial_price, 200.0, places=2)
        self.assertAlmostEqual(config.volatility, 0.03, places=4)
    
    def test_generate_series(self):
        """Test OHLC data generation."""
        # Generate data
        data = self.generator.generate_series(100)
        
        # Check length
        self.assertEqual(len(data), 100)
        
        # Check data structure
        self.assertIsInstance(data, list)
        self.assertIsInstance(data[0], dict)
        
        # Check required fields
        required_fields = ['timestamp', 'open', 'high', 'low', 'close', 'volume']
        for field in required_fields:
            self.assertIn(field, data[0])
        
        # Check data validity
        for bar in data:
            # High should be >= Low
            self.assertGreaterEqual(bar['high'], bar['low'])
            # High should be >= Open and Close
            self.assertGreaterEqual(bar['high'], bar['open'])
            self.assertGreaterEqual(bar['high'], bar['close'])
            # Low should be <= Open and Close
            self.assertLessEqual(bar['low'], bar['open'])
            self.assertLessEqual(bar['low'], bar['close'])
            # Volume should be positive
            self.assertGreater(bar['volume'], 0)
    
    def test_generate_ticks(self):
        """Test tick data generation."""
        # Generate ticks (spread parameter not supported)
        ticks = self.generator.generate_ticks(50)
        
        # Check length
        self.assertEqual(len(ticks), 50)
        
        # Check data structure
        self.assertIsInstance(ticks, list)
        self.assertIsInstance(ticks[0], dict)
        
        # Check required fields
        required_fields = ['timestamp', 'bid', 'ask', 'spread', 'volume']
        for field in required_fields:
            self.assertIn(field, ticks[0])
        
        # Check data validity
        for tick in ticks:
            # Ask should be > Bid
            self.assertGreater(tick['ask'], tick['bid'])
            # Spread should be positive
            self.assertGreater(tick['spread'], 0)
            # Spread should approximately equal ask - bid
            self.assertAlmostEqual(tick['spread'], tick['ask'] - tick['bid'], places=6)
            # Volume should be positive
            self.assertGreater(tick['volume'], 0)
    
    def test_generate_series_between(self):
        """Test generation between timestamps."""
        import time
        
        # Get current time and future time
        start = int(time.time())
        end = start + 3600  # 1 hour later
        
        # Generate data
        data = self.generator.generate_series_between(start, end)
        
        # Check that data was generated
        self.assertIsInstance(data, list)
        self.assertGreater(len(data), 0)
        
        # Check timestamps are within range
        for bar in data:
            self.assertGreaterEqual(bar['timestamp'], start)
            self.assertLessEqual(bar['timestamp'], end)
    
    def test_deterministic_generation(self):
        """Test that same seed produces same results."""
        # Create two generators with same seed
        gen1 = mds.MarketDataGenerator(seed=123)
        gen2 = mds.MarketDataGenerator(seed=123)
        
        # Generate data
        data1 = gen1.generate_series(10)
        data2 = gen2.generate_series(10)
        
        # Check that data is identical
        self.assertEqual(len(data1), len(data2))
        for bar1, bar2 in zip(data1, data2):
            self.assertAlmostEqual(bar1['open'], bar2['open'], places=10)
            self.assertAlmostEqual(bar1['close'], bar2['close'], places=10)
    
    # Export methods are not available in Python bindings
    # These features are only available in the Rust library
    
    def test_set_seed(self):
        """Test seed setting functionality."""
        # Generate initial data
        data1 = self.generator.generate_series(5)
        
        # Set new seed
        self.generator.set_seed(999)
        data2 = self.generator.generate_series(5)
        
        # Set same seed again
        self.generator.set_seed(999)
        data3 = self.generator.generate_series(5)
        
        # Data1 and data2 should be different
        self.assertNotEqual(data1[0]['close'], data2[0]['close'])
        
        # Data2 and data3 should be identical
        self.assertAlmostEqual(data2[0]['close'], data3[0]['close'], places=10)
    
    def test_reset(self):
        """Test generator reset functionality."""
        # Generate some data
        initial_data = self.generator.generate_series(5)
        
        # Reset generator
        self.generator.reset()
        
        # Generate new data
        reset_data = self.generator.generate_series(5)
        
        # First values should be similar (same initial price)
        self.assertAlmostEqual(
            initial_data[0]['open'], 
            reset_data[0]['open'], 
            places=2
        )
    
    def test_preset_configs(self):
        """Test preset configuration functions."""
        # Test volatile config
        volatile = mds.volatile_config()
        self.assertIsNotNone(volatile)
        config = volatile.config
        self.assertGreater(config.volatility, 0.03)  # Should be high volatility
        
        # Test stable config
        stable = mds.stable_config()
        self.assertIsNotNone(stable)
        config = stable.config
        self.assertLess(config.volatility, 0.01)  # Should be low volatility
        
        # Test bull market config
        bull = mds.bull_market_config()
        self.assertIsNotNone(bull)
        config = bull.config
        self.assertGreater(config.trend_strength, 0)  # Should have positive trend
        
        # Test bear market config
        bear = mds.bear_market_config()
        self.assertIsNotNone(bear)
        config = bear.config
        # Bear market has positive trend_strength with Bearish direction
        self.assertGreater(config.trend_strength, 0)  # Strength is always positive
        self.assertIn("Bearish", config.trend_direction)  # Direction indicates bear
    
    def test_price_boundaries(self):
        """Test min/max price boundaries."""
        # Create generator with boundaries
        gen = mds.MarketDataGenerator(
            initial_price=100.0,
            min_price=90.0,
            max_price=110.0,
            volatility=0.1,  # High volatility to test boundaries
            seed=42
        )
        
        # Generate data
        data = gen.generate_series(1000)
        
        # Check all prices are within boundaries
        for bar in data:
            self.assertGreaterEqual(bar['low'], 90.0)
            self.assertLessEqual(bar['high'], 110.0)
    
    def test_different_intervals(self):
        """Test different time intervals."""
        intervals = ["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
        
        for interval in intervals:
            gen = mds.MarketDataGenerator(interval=interval)
            data = gen.generate_series(10)
            
            # Check data was generated
            self.assertEqual(len(data), 10)
            
            # Check timestamps are properly spaced
            if len(data) > 1:
                # Calculate expected seconds between bars
                expected_seconds = {
                    "1m": 60,
                    "5m": 300,
                    "15m": 900,
                    "30m": 1800,
                    "1h": 3600,
                    "4h": 14400,
                    "1d": 86400
                }[interval]
                
                # Timestamps are in milliseconds, not seconds
                actual_ms = data[1]['timestamp'] - data[0]['timestamp']
                expected_ms = expected_seconds * 1000
                self.assertEqual(actual_ms, expected_ms)
    


class TestDataQuality(unittest.TestCase):
    """Test cases for data quality and statistical properties."""
    
    def test_price_continuity(self):
        """Test that prices are continuous between bars."""
        gen = mds.MarketDataGenerator(seed=42)
        data = gen.generate_series(100)
        
        for i in range(1, len(data)):
            prev_close = data[i-1]['close']
            curr_open = data[i]['open']
            
            # Open of current bar should equal close of previous bar
            self.assertAlmostEqual(prev_close, curr_open, places=10)
    
    def test_volume_properties(self):
        """Test volume generation properties."""
        gen = mds.MarketDataGenerator(
            volume_base=1000000,  # Must be integer
            volume_volatility=0.2,
            seed=42
        )
        data = gen.generate_series(1000)
        
        volumes = [bar['volume'] for bar in data]
        
        # All volumes should be positive
        self.assertTrue(all(v > 0 for v in volumes))
        
        # Average volume should be close to base volume
        avg_volume = sum(volumes) / len(volumes)
        self.assertAlmostEqual(avg_volume, 1000000.0, delta=100000)
    
    def test_trend_effect(self):
        """Test that trend parameter affects price direction."""
        # Upward trend
        gen_up = mds.MarketDataGenerator(
            initial_price=100.0,
            trend=0.001,
            volatility=0.001,  # Low volatility to see trend clearly
            seed=42
        )
        data_up = gen_up.generate_series(100)
        
        # Downward trend
        gen_down = mds.MarketDataGenerator(
            initial_price=100.0,
            trend=-0.001,
            volatility=0.001,
            seed=42
        )
        data_down = gen_down.generate_series(100)
        
        # Check final prices
        final_up = data_up[-1]['close']
        final_down = data_down[-1]['close']
        
        # Upward trend should result in higher final price
        self.assertGreater(final_up, 100.0)
        # Downward trend should result in lower final price
        self.assertLess(final_down, 100.0)


if __name__ == "__main__":
    unittest.main()