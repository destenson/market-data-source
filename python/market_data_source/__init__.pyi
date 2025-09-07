"""Type stubs for market_data_source"""

from typing import List, Dict, Optional, Any

class GeneratorConfig:
    """Configuration for market data generation"""
    @property
    def initial_price(self) -> float: ...
    @property
    def volatility(self) -> float: ...
    @property
    def trend(self) -> float: ...
    @property
    def min_price(self) -> Optional[float]: ...
    @property
    def max_price(self) -> Optional[float]: ...
    @property
    def volume_base(self) -> float: ...
    @property
    def volume_volatility(self) -> float: ...
    @property
    def interval(self) -> str: ...
    @property
    def seed(self) -> Optional[int]: ...
    def __repr__(self) -> str: ...

class MarketDataGenerator:
    """Generator for synthetic market data"""
    
    def __init__(self, **kwargs: Any) -> None:
        """
        Create a new MarketDataGenerator.
        
        Keyword Args:
            initial_price: Starting price (default: 100.0)
            volatility: Price volatility (default: 0.02)
            trend: Price trend/drift (default: 0.0)
            min_price: Minimum price boundary
            max_price: Maximum price boundary
            volume_base: Base volume (default: 1000.0)
            volume_volatility: Volume volatility (default: 0.1)
            interval: Time interval ("1m", "5m", "15m", "30m", "1h", "4h", "1d")
            seed: Random seed for reproducibility
        """
        ...
    
    def generate_series(self, count: int) -> List[Dict[str, Any]]:
        """
        Generate OHLC data series.
        
        Args:
            count: Number of bars to generate
            
        Returns:
            List of dictionaries containing OHLC data with keys:
            - timestamp: Unix timestamp
            - open: Opening price
            - high: High price
            - low: Low price
            - close: Closing price
            - volume: Trading volume
        """
        ...
    
    def generate_ticks(self, count: int, spread: Optional[float] = None) -> List[Dict[str, Any]]:
        """
        Generate tick data.
        
        Args:
            count: Number of ticks to generate
            spread: Bid-ask spread (default: 0.01)
            
        Returns:
            List of dictionaries containing tick data with keys:
            - timestamp: Unix timestamp
            - bid: Bid price
            - ask: Ask price
            - spread: Bid-ask spread
            - volume: Trading volume
        """
        ...
    
    def generate_series_between(self, start: int, end: int) -> List[Dict[str, Any]]:
        """
        Generate data between two timestamps.
        
        Args:
            start: Start timestamp (Unix seconds)
            end: End timestamp (Unix seconds)
            
        Returns:
            List of OHLC dictionaries
        """
        ...
    
    def to_csv(self, path: str, count: int) -> None:
        """Export data to CSV file."""
        ...
    
    def to_json(self, path: str, count: int, lines: Optional[bool] = None) -> None:
        """Export data to JSON file."""
        ...
    
    def to_png(self, path: str, count: int, **kwargs: Any) -> None:
        """
        Export data to PNG chart.
        
        Keyword Args:
            width: Chart width in pixels
            height: Chart height in pixels
            title: Chart title
            volume: Show volume bars
        """
        ...
    
    @property
    def config(self) -> GeneratorConfig:
        """Get current configuration."""
        ...
    
    def set_seed(self, seed: int) -> None:
        """Set new random seed."""
        ...
    
    def reset(self) -> None:
        """Reset generator to initial state."""
        ...
    
    def __repr__(self) -> str: ...

def volatile_config() -> MarketDataGenerator:
    """Create generator with volatile market configuration."""
    ...

def stable_config() -> MarketDataGenerator:
    """Create generator with stable market configuration."""
    ...

def bull_market_config() -> MarketDataGenerator:
    """Create generator with bull market configuration."""
    ...

def bear_market_config() -> MarketDataGenerator:
    """Create generator with bear market configuration."""
    ...

def create_generator(**kwargs: Any) -> MarketDataGenerator:
    """Convenience function to create a generator with keyword arguments."""
    ...

__version__: str
__all__: List[str]