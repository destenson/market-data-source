# Python Setup Guide

## Quick Start with UV (Recommended)

We recommend using [uv](https://github.com/astral-sh/uv) for the fastest and easiest Python setup experience.

### Install UV

```bash
# Windows (PowerShell)
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"

# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### Build and Install

```bash
# Install maturin for building
uv pip install maturin

# Build and install the package in development mode
# With synthetic data generation only
uv run maturin develop --features python,synthetic

# With live data capabilities  
uv run maturin develop --features python,live

# With all features
uv run maturin develop --features python,synthetic,live

# Or build a wheel
uv run maturin build --features python,synthetic,live --release
```

### Run Examples

```bash
# Run basic example
uv run python examples/python/basic_usage.py

# Run pandas integration example
uv pip install pandas matplotlib
uv run python examples/python/pandas_integration.py
```

### Run Tests

```bash
# Install test dependencies
uv pip install pytest pytest-cov

# Run tests
uv run pytest tests/python/
```

## Alternative: Traditional Setup

If you prefer traditional Python tools:

### Using pip and venv

```bash
# Create virtual environment
python -m venv venv

# Activate it
# Windows
venv\Scripts\activate
# macOS/Linux
source venv/bin/activate

# Install maturin
pip install maturin

# Build and install
maturin develop --features python

# Run examples
python examples/python/basic_usage.py
```

## Using in Your Project

### With UV

```bash
# In your project directory
uv pip install market-data-source

# Or install from local wheel
uv pip install path/to/market_data_source-0.2.0-*.whl
```

### In Python

```python
import market_data_source as mds

# Quick start
generator = mds.create_generator(
    initial_price=100.0,
    volatility=0.02,
    seed=42
)

# Generate data
data = generator.generate_series(100)

# Export to files
generator.to_csv("output.csv", count=1000)
generator.to_json("output.json", count=1000)
generator.to_png("chart.png", count=500)
```

## Development Workflow

### Using UV for Development

```bash
# Install all development dependencies
uv pip install maturin pytest pytest-cov black ruff mypy

# Format code
uv run black examples/python/ tests/python/

# Lint code
uv run ruff check examples/python/ tests/python/

# Type check (after installing types)
uv pip install pandas-stubs types-matplotlib
uv run mypy examples/python/

# Run tests with coverage
uv run pytest tests/python/ --cov=market_data_source --cov-report=html

# Build release wheel
uv run maturin build --features python --release
```

### Jupyter Notebook Usage

```bash
# Install Jupyter
uv pip install jupyter pandas matplotlib

# Install the package
uv run maturin develop --features python

# Start Jupyter
uv run jupyter notebook
```

Then in a notebook:

```python
import market_data_source as mds
import pandas as pd
import matplotlib.pyplot as plt

# Generate data
gen = mds.MarketDataGenerator(volatility=0.03)
data = gen.generate_series(1000)

# Convert to DataFrame
df = pd.DataFrame(data)
df['datetime'] = pd.to_datetime(df['timestamp'], unit='s')
df.set_index('datetime', inplace=True)

# Plot
df['close'].plot(figsize=(12, 6))
plt.show()
```

## Troubleshooting

### Common Issues

1. **ImportError: No module named 'market_data_source'**
   - Solution: Run `uv run maturin develop --features python`

2. **Rust not found**
   - Solution: Install Rust from https://rustup.rs/

3. **Build fails on Windows**
   - Solution: Install Visual Studio Build Tools

4. **UV not found**
   - Solution: Restart your terminal after installing uv

### Performance Tips

- Use `--release` flag for production builds: `uv run maturin build --features python --release`
- The Rust implementation is 10-100x faster than pure Python alternatives
- Generate large datasets in batches for better memory usage

## Publishing to PyPI

```bash
# Build release wheels for all platforms
uv run maturin build --features python --release

# Upload to PyPI (requires account)
uv pip install twine
uv run twine upload target/wheels/*
```

## CI/CD with GitHub Actions

See `.github/workflows/python.yml` for automated building and testing across platforms.