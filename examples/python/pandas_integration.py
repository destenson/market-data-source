#!/usr/bin/env python3
"""
Pandas integration example for market_data_source.

This example demonstrates how to use the library with pandas DataFrames for:
- Converting generated data to DataFrames
- Technical analysis calculations
- Data visualization with matplotlib
- Statistical analysis
"""

import market_data_source as mds
import pandas as pd
import numpy as np
from datetime import datetime, timedelta

# Optional: Import matplotlib for visualization
try:
    import matplotlib.pyplot as plt
    import matplotlib.dates as mdates
    MATPLOTLIB_AVAILABLE = True
except ImportError:
    MATPLOTLIB_AVAILABLE = False
    print("Note: Install matplotlib for visualization features")


def create_dataframe(generator, count=1000):
    """Convert generated OHLC data to pandas DataFrame."""
    data = generator.generate_series(count)
    
    # Create DataFrame
    df = pd.DataFrame(data)
    
    # Convert timestamp to datetime
    df['datetime'] = pd.to_datetime(df['timestamp'], unit='s')
    df.set_index('datetime', inplace=True)
    
    # Ensure numeric types
    for col in ['open', 'high', 'low', 'close', 'volume']:
        df[col] = pd.to_numeric(df[col])
    
    return df


def add_technical_indicators(df):
    """Add common technical indicators to the DataFrame."""
    
    # Simple Moving Averages
    df['SMA_20'] = df['close'].rolling(window=20).mean()
    df['SMA_50'] = df['close'].rolling(window=50).mean()
    df['SMA_200'] = df['close'].rolling(window=200).mean()
    
    # Exponential Moving Average
    df['EMA_12'] = df['close'].ewm(span=12, adjust=False).mean()
    df['EMA_26'] = df['close'].ewm(span=26, adjust=False).mean()
    
    # MACD
    df['MACD'] = df['EMA_12'] - df['EMA_26']
    df['MACD_signal'] = df['MACD'].ewm(span=9, adjust=False).mean()
    df['MACD_histogram'] = df['MACD'] - df['MACD_signal']
    
    # Bollinger Bands
    bb_period = 20
    bb_std = 2
    df['BB_middle'] = df['close'].rolling(window=bb_period).mean()
    bb_std_dev = df['close'].rolling(window=bb_period).std()
    df['BB_upper'] = df['BB_middle'] + (bb_std_dev * bb_std)
    df['BB_lower'] = df['BB_middle'] - (bb_std_dev * bb_std)
    
    # RSI (Relative Strength Index)
    delta = df['close'].diff()
    gain = (delta.where(delta > 0, 0)).rolling(window=14).mean()
    loss = (-delta.where(delta < 0, 0)).rolling(window=14).mean()
    rs = gain / loss
    df['RSI'] = 100 - (100 / (1 + rs))
    
    # Volume indicators
    df['Volume_SMA'] = df['volume'].rolling(window=20).mean()
    df['Volume_ratio'] = df['volume'] / df['Volume_SMA']
    
    # Price changes
    df['returns'] = df['close'].pct_change()
    df['log_returns'] = np.log(df['close'] / df['close'].shift(1))
    
    return df


def calculate_statistics(df):
    """Calculate statistical metrics for the data."""
    stats = {
        'Total bars': len(df),
        'Date range': f"{df.index[0]} to {df.index[-1]}",
        'Starting price': df['close'].iloc[0],
        'Ending price': df['close'].iloc[-1],
        'Min price': df['low'].min(),
        'Max price': df['high'].max(),
        'Total return': (df['close'].iloc[-1] / df['close'].iloc[0] - 1) * 100,
        'Mean return': df['returns'].mean() * 100,
        'Volatility (std)': df['returns'].std() * 100,
        'Sharpe ratio': df['returns'].mean() / df['returns'].std() * np.sqrt(252) if df['returns'].std() > 0 else 0,
        'Max drawdown': calculate_max_drawdown(df['close']),
        'Average volume': df['volume'].mean(),
        'Total volume': df['volume'].sum(),
    }
    return stats


def calculate_max_drawdown(prices):
    """Calculate maximum drawdown percentage."""
    cumulative_returns = (1 + prices.pct_change()).cumprod()
    running_max = cumulative_returns.expanding().max()
    drawdown = (cumulative_returns - running_max) / running_max
    return drawdown.min() * 100


def plot_data(df, title="Market Data Analysis"):
    """Create a comprehensive plot of the market data."""
    if not MATPLOTLIB_AVAILABLE:
        print("Matplotlib not available, skipping plots")
        return
    
    fig, axes = plt.subplots(4, 1, figsize=(12, 10), sharex=True)
    
    # Price and moving averages
    ax1 = axes[0]
    ax1.plot(df.index, df['close'], label='Close', linewidth=1, color='black')
    ax1.plot(df.index, df['SMA_20'], label='SMA 20', alpha=0.7)
    ax1.plot(df.index, df['SMA_50'], label='SMA 50', alpha=0.7)
    ax1.fill_between(df.index, df['BB_upper'], df['BB_lower'], alpha=0.1, color='gray')
    ax1.set_ylabel('Price ($)')
    ax1.legend(loc='best')
    ax1.set_title(title)
    ax1.grid(True, alpha=0.3)
    
    # Volume
    ax2 = axes[1]
    colors = ['g' if c > o else 'r' for c, o in zip(df['close'], df['open'])]
    ax2.bar(df.index, df['volume'], color=colors, alpha=0.5)
    ax2.plot(df.index, df['Volume_SMA'], label='Volume SMA', color='blue', alpha=0.7)
    ax2.set_ylabel('Volume')
    ax2.legend(loc='best')
    ax2.grid(True, alpha=0.3)
    
    # MACD
    ax3 = axes[2]
    ax3.plot(df.index, df['MACD'], label='MACD', color='blue')
    ax3.plot(df.index, df['MACD_signal'], label='Signal', color='red')
    ax3.bar(df.index, df['MACD_histogram'], label='Histogram', alpha=0.3)
    ax3.set_ylabel('MACD')
    ax3.legend(loc='best')
    ax3.grid(True, alpha=0.3)
    ax3.axhline(y=0, color='black', linestyle='-', alpha=0.3)
    
    # RSI
    ax4 = axes[3]
    ax4.plot(df.index, df['RSI'], label='RSI', color='purple')
    ax4.axhline(y=70, color='r', linestyle='--', alpha=0.5, label='Overbought')
    ax4.axhline(y=30, color='g', linestyle='--', alpha=0.5, label='Oversold')
    ax4.set_ylabel('RSI')
    ax4.set_xlabel('Date')
    ax4.legend(loc='best')
    ax4.grid(True, alpha=0.3)
    ax4.set_ylim(0, 100)
    
    # Format x-axis
    for ax in axes:
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        ax.xaxis.set_major_locator(mdates.AutoDateLocator())
    
    plt.xticks(rotation=45)
    plt.tight_layout()
    
    return fig


def main():
    print("Market Data Source - Pandas Integration Example")
    print("=" * 50)
    
    # Create generator with specific configuration
    generator = mds.MarketDataGenerator(
        initial_price=100.0,
        volatility=0.025,
        trend=0.0002,  # Slight upward trend
        volume_base=1000000.0,
        volume_volatility=0.15,
        interval="1h",
        seed=42
    )
    
    # Generate data and convert to DataFrame
    print("\n1. Generating market data...")
    df = create_dataframe(generator, count=1000)
    print(f"Generated {len(df)} bars from {df.index[0]} to {df.index[-1]}")
    
    # Add technical indicators
    print("\n2. Adding technical indicators...")
    df = add_technical_indicators(df)
    print("Added: SMA, EMA, MACD, Bollinger Bands, RSI, Volume indicators")
    
    # Display first few rows
    print("\n3. Sample data (first 5 rows):")
    print(df[['open', 'high', 'low', 'close', 'volume', 'SMA_20', 'RSI']].head())
    
    # Calculate statistics
    print("\n4. Statistical Analysis:")
    stats = calculate_statistics(df)
    for key, value in stats.items():
        if isinstance(value, float):
            print(f"  {key}: {value:.2f}")
        else:
            print(f"  {key}: {value}")
    
    # Correlation analysis
    print("\n5. Correlation Analysis:")
    corr_cols = ['returns', 'volume', 'RSI', 'MACD']
    correlation = df[corr_cols].corr()
    print(correlation.round(3))
    
    # Trading signals example
    print("\n6. Sample Trading Signals:")
    # Golden cross (SMA 20 crosses above SMA 50)
    df['golden_cross'] = (
        (df['SMA_20'] > df['SMA_50']) & 
        (df['SMA_20'].shift(1) <= df['SMA_50'].shift(1))
    )
    
    # Death cross (SMA 20 crosses below SMA 50)
    df['death_cross'] = (
        (df['SMA_20'] < df['SMA_50']) & 
        (df['SMA_20'].shift(1) >= df['SMA_50'].shift(1))
    )
    
    golden_crosses = df[df['golden_cross']]
    death_crosses = df[df['death_cross']]
    
    print(f"  Golden crosses: {len(golden_crosses)}")
    print(f"  Death crosses: {len(death_crosses)}")
    
    if len(golden_crosses) > 0:
        print(f"  Last golden cross: {golden_crosses.index[-1]}")
    if len(death_crosses) > 0:
        print(f"  Last death cross: {death_crosses.index[-1]}")
    
    # Save processed data
    print("\n7. Saving processed data...")
    df.to_csv("output/pandas_data.csv")
    print("  ✓ Saved to output/pandas_data.csv")
    
    # Create visualization
    if MATPLOTLIB_AVAILABLE:
        print("\n8. Creating visualization...")
        fig = plot_data(df, title="Market Data with Technical Indicators")
        fig.savefig("output/pandas_analysis.png", dpi=100, bbox_inches='tight')
        print("  ✓ Saved plot to output/pandas_analysis.png")
        plt.show()
    
    # Resample to different timeframes
    print("\n9. Resampling data...")
    daily_df = df.resample('1D').agg({
        'open': 'first',
        'high': 'max',
        'low': 'min',
        'close': 'last',
        'volume': 'sum'
    }).dropna()
    print(f"  Daily bars: {len(daily_df)}")
    
    weekly_df = df.resample('1W').agg({
        'open': 'first',
        'high': 'max',
        'low': 'min',
        'close': 'last',
        'volume': 'sum'
    }).dropna()
    print(f"  Weekly bars: {len(weekly_df)}")
    
    print("\n✅ Pandas integration example completed!")


if __name__ == "__main__":
    import os
    
    # Create output directory if it doesn't exist
    os.makedirs("output", exist_ok=True)
    
    main()