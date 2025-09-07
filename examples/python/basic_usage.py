#!/usr/bin/env python3
"""
Basic usage example for market_data_source Python bindings.

This example demonstrates the core functionality of the library including:
- Creating a generator with custom configuration
- Generating OHLC and tick data
- Exporting data to various formats
- Using preset configurations
"""

import market_data_source as mds
from datetime import datetime


def main():
    print("Market Data Source - Basic Python Example")
    print("=" * 50)
    
    # Create a generator with custom configuration
    generator = mds.MarketDataGenerator(
        initial_price=100.0,
        volatility=0.02,
        trend=0.0001,
        volume_base=1000000,  # Integer, not float
        interval="1m",
        seed=42  # For reproducible results
    )
    
    # Generate OHLC data
    print("\n1. Generating OHLC data...")
    ohlc_data = generator.generate_series(10)
    
    print(f"Generated {len(ohlc_data)} OHLC bars")
    print("\nFirst 3 bars:")
    for i, bar in enumerate(ohlc_data[:3], 1):
        dt = datetime.fromtimestamp(bar['timestamp'] / 1000)  # Convert from milliseconds
        print(f"  Bar {i} ({dt.strftime('%Y-%m-%d %H:%M:%S')}):")
        print(f"    Open:  ${bar['open']:.2f}")
        print(f"    High:  ${bar['high']:.2f}")
        print(f"    Low:   ${bar['low']:.2f}")
        print(f"    Close: ${bar['close']:.2f}")
        print(f"    Volume: {bar['volume']:,.0f}")
    
    # Generate tick data
    print("\n2. Generating tick data...")
    tick_data = generator.generate_ticks(5)  # No spread parameter
    
    print(f"Generated {len(tick_data)} ticks")
    print("\nFirst 3 ticks:")
    for i, tick in enumerate(tick_data[:3], 1):
        dt = datetime.fromtimestamp(tick['timestamp'] / 1000)  # Convert from milliseconds
        print(f"  Tick {i} ({dt.strftime('%H:%M:%S')}):")
        print(f"    Bid: ${tick['bid']:.2f}, Ask: ${tick['ask']:.2f}, Spread: ${tick['spread']:.4f}")
    
    # Export data to files
    print("\n3. Exporting data to files...")
    generator.to_csv("output/basic_data.csv", count=100)
    print("  [OK] Exported to CSV: output/basic_data.csv")
    
    generator.to_json("output/basic_data.json", count=100)
    print("  [OK] Exported to JSON: output/basic_data.json")
    
    generator.to_png("output/basic_chart.png", count=100, 
                     width=1200, height=800, title="Basic Market Data")
    print("  [OK] Exported to PNG: output/basic_chart.png")
    
    # Using preset configurations
    print("\n4. Using preset configurations...")
    
    # Volatile market
    volatile_gen = mds.volatile_config()
    volatile_data = volatile_gen.generate_series(5)
    print(f"  Volatile market config: {volatile_gen.config}")
    
    # Stable market
    stable_gen = mds.stable_config()
    stable_data = stable_gen.generate_series(5)
    print(f"  Stable market config: {stable_gen.config}")
    
    # Bull market
    bull_gen = mds.bull_market_config()
    bull_data = bull_gen.generate_series(5)
    print(f"  Bull market config: {bull_gen.config}")
    
    # Access configuration
    print("\n5. Accessing configuration...")
    config = generator.config
    print(f"  Initial price: ${config.initial_price:.2f}")
    print(f"  Volatility: {config.volatility:.4f}")
    print(f"  Trend strength: {config.trend_strength:.6f}")
    print(f"  Time interval: {config.time_interval}")
    
    # Reset and change seed
    print("\n6. Resetting generator...")
    generator.reset()
    generator.set_seed(123)
    new_data = generator.generate_series(5)
    print(f"  Generated {len(new_data)} bars after reset with new seed")
    
    print("\n[SUCCESS] Example completed successfully!")


if __name__ == "__main__":
    import os
    
    # Create output directory if it doesn't exist
    os.makedirs("output", exist_ok=True)
    
    main()