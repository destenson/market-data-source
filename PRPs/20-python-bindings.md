# PRP-20: Python Bindings Implementation

## Overview
Implement PyO3-based Python bindings for the market-data-source library to enable Python developers to use the library directly from Python code. This will provide access to the high-performance Rust implementation while maintaining a Pythonic API.

## Motivation
- **Python Ecosystem**: Enable integration with pandas, NumPy, and ML frameworks
- **Data Science Workflows**: Support quantitative trading and backtesting frameworks
- **Ease of Use**: Provide simple Python API for non-Rust developers
- **Performance**: Leverage Rust's speed from Python applications

## Requirements

### Core Functionality
1. **MarketDataGenerator Class**: Python wrapper for Rust generator
2. **Configuration**: Python-friendly configuration methods
3. **Data Generation**: Methods returning Python-native data structures
4. **Export Methods**: Direct export to files from Python
5. **Type Safety**: Proper type hints and error handling

### Python API Design
```python
import market_data_source as mds

# Create generator with configuration
generator = mds.MarketDataGenerator(
    initial_price=100.0,
    volatility=0.02,
    trend=0.0001,
    seed=42
)

# Generate OHLC data as list of dicts
data = generator.generate_series(count=100)

# Generate tick data
ticks = generator.generate_ticks(count=1000)

# Direct export methods
generator.to_csv("output.csv", count=500)
generator.to_json("output.json", count=500)

# Access configuration
config = generator.config()
```

## Implementation Plan

### Phase 1: Setup Build System
1. Add PyO3 and maturin dependencies to Cargo.toml
2. Configure project for Python extension
3. Setup GitHub Actions for wheel building
4. Create pyproject.toml for pip installation

### Phase 2: Core Python Module
1. Create src/python.rs module with PyO3 bindings
2. Implement PyMarketDataGenerator wrapper class
3. Add configuration builder methods
4. Implement error conversion from Rust to Python

### Phase 3: Data Generation Methods
1. `generate_series(count: int) -> List[Dict]` - OHLC data
2. `generate_ticks(count: int) -> List[Dict]` - Tick data
3. `generate_series_between(start: str, end: str) -> List[Dict]` - Time range
4. Proper datetime conversion for timestamps

### Phase 4: Export Methods
1. `to_csv(path: str, count: int)` - Export to CSV
2. `to_json(path: str, count: int)` - Export to JSON
3. `to_png(path: str, count: int)` - Generate charts
4. Error handling for file operations

### Phase 5: Python Package Setup
1. Create proper package structure
2. Add __init__.py with public API
3. Type stub files (.pyi) for IDE support
4. Setup.py and requirements.txt

### Phase 6: Documentation & Examples
1. Python docstrings for all methods
2. Example scripts in examples/python/
3. Update README with Python usage
4. Jupyter notebook examples

## Technical Details

### Dependencies
```toml
[dependencies]
pyo3 = { version = "0.23", features = ["extension-module", "chrono"] }

[build-dependencies]
maturin = "1.5"
```

### Build Configuration
```toml
[lib]
name = "market_data_source"
crate-type = ["cdylib", "rlib"]

[package.metadata.maturin]
python-source = "python"
```

### Type Conversions
- Decimal → float (with precision handling)
- DateTime<Utc> → Python datetime
- Vec<OHLC> → List[Dict]
- GeneratorConfig → Dict

### Error Handling
- Convert Rust Results to Python exceptions
- Proper error messages for debugging
- Type validation for Python inputs

## Testing Requirements

### Python Unit Tests
1. Test data generation methods
2. Test configuration options
3. Test export functionality
4. Test error conditions
5. Performance benchmarks

### Integration Tests
1. Test with pandas DataFrames
2. Test with NumPy arrays
3. Test with common plotting libraries
4. Test with backtrading frameworks

## Success Criteria
1. ✅ Python package installable via pip
2. ✅ All core generation methods accessible
3. ✅ Export methods working from Python
4. ✅ Type hints for IDE support
5. ✅ Examples demonstrating usage
6. ✅ Tests passing in Python
7. ✅ Documentation updated

## Example Python Usage

### Basic Generation
```python
import market_data_source as mds

# Create generator
gen = mds.MarketDataGenerator(initial_price=100.0)

# Generate 1000 OHLC bars
data = gen.generate_series(1000)

# Access individual bars
for bar in data[:10]:
    print(f"Open: {bar['open']}, High: {bar['high']}, "
          f"Low: {bar['low']}, Close: {bar['close']}")
```

### With Pandas
```python
import market_data_source as mds
import pandas as pd

gen = mds.MarketDataGenerator(volatility=0.03)
data = gen.generate_series(5000)

# Convert to DataFrame
df = pd.DataFrame(data)
df['timestamp'] = pd.to_datetime(df['timestamp'])
df.set_index('timestamp', inplace=True)

# Use with technical analysis
df['SMA_20'] = df['close'].rolling(20).mean()
df['SMA_50'] = df['close'].rolling(50).mean()
```

### Export Example
```python
import market_data_source as mds

gen = mds.MarketDataGenerator(
    initial_price=50000.0,  # Bitcoin-like price
    volatility=0.05,         # High volatility
    trend=0.001              # Upward trend
)

# Generate and export
gen.to_csv("btc_data.csv", count=10000)
gen.to_json("btc_data.json", count=10000)
gen.to_png("btc_chart.png", count=500)
```

## Files to Create/Modify

### New Files
1. `src/python.rs` - PyO3 bindings
2. `pyproject.toml` - Python package config
3. `python/market_data_source/__init__.py` - Python module
4. `python/market_data_source/__init__.pyi` - Type stubs
5. `examples/python/basic_usage.py` - Example script
6. `examples/python/pandas_integration.py` - Pandas example
7. `tests/python/test_generator.py` - Python tests

### Modified Files
1. `Cargo.toml` - Add PyO3 dependencies
2. `src/lib.rs` - Add Python module
3. `README.md` - Add Python usage section
4. `.github/workflows/` - Add Python CI/CD

## Potential Challenges
1. **Decimal Precision**: Converting Decimal to float may lose precision
2. **Performance**: Python GIL may limit parallelism
3. **Memory**: Large datasets may require streaming
4. **Type Safety**: Maintaining type hints accuracy
5. **Cross-platform**: Building wheels for all platforms

## Future Enhancements
1. NumPy array support for better performance
2. Async/await support for streaming
3. Direct pandas DataFrame generation
4. Pickle support for serialization
5. Jupyter widgets for interactive generation