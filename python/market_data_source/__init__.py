"""
Market Data Source - High-performance synthetic market data generation

This library provides tools for generating realistic synthetic market data
for backtesting, research, and development purposes.
"""

from market_data_source._market_data_source import (
    MarketDataGenerator,
    GeneratorConfig,
    volatile_config,
    stable_config,
    bull_market_config,
    bear_market_config,
    __version__,
)

__all__ = [
    "MarketDataGenerator",
    "GeneratorConfig",
    "volatile_config",
    "stable_config",
    "bull_market_config",
    "bear_market_config",
    "__version__",
]

# Convenience function for quick start
def create_generator(**kwargs):
    """
    Create a new MarketDataGenerator with keyword arguments.
    
    Parameters:
    -----------
    initial_price : float, optional
        Starting price (default: 100.0)
    volatility : float, optional
        Price volatility (default: 0.02)
    trend : float, optional
        Price trend/drift (default: 0.0)
    min_price : float, optional
        Minimum price boundary
    max_price : float, optional
        Maximum price boundary
    volume_base : float, optional
        Base volume (default: 1000.0)
    volume_volatility : float, optional
        Volume volatility (default: 0.1)
    interval : str, optional
        Time interval: "1m", "5m", "15m", "30m", "1h", "4h", "1d" (default: "1m")
    seed : int, optional
        Random seed for reproducibility
    
    Returns:
    --------
    MarketDataGenerator
        A configured market data generator instance
    
    Example:
    --------
    >>> gen = create_generator(initial_price=150.0, volatility=0.03)
    >>> data = gen.generate_series(100)
    """
    return MarketDataGenerator(**kwargs)