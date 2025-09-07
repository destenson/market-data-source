# Contributing to Market Data Source

Thank you for your interest in contributing to Market Data Source! This document provides guidelines and instructions for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.88.0 or later
- Python 3.8+ (for Python bindings)
- Git

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/destenson/market-data-source.git
cd market-data-source
```

2. Install Rust dependencies:
```bash
cargo build
```

3. For Python development:
```bash
pip install maturin
maturin develop
```

## CI/CD Requirements

All pull requests must pass the following CI checks:

### 1. Tests
- **Rust tests**: Must pass on Linux, macOS, and Windows
- **Python tests**: Must pass on Python 3.8-3.12
- **Feature tests**: Various feature combinations are tested

Run locally with:
```bash
# Basic tests
cargo test

# All features (use -j 1 on Windows if memory issues occur)
cargo test --all-features

# Specific features
cargo test --features csv_export,json_export
```

### 2. Code Quality
- **Formatting**: Code must be formatted with `rustfmt`
- **Linting**: Must pass `clippy` with no warnings
- **Documentation**: Must build without warnings

Run locally with:
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-features --all-targets -- -D warnings

# Build documentation
cargo doc --no-deps --all-features
```

### 3. Python Bindings
If modifying Python bindings:
```bash
# Build wheel
maturin build --release

# Run Python tests
python -c "import market_data_source; print(market_data_source.__version__)"
```

## Workflow Overview

### GitHub Actions Workflows

1. **test.yml**: Runs on every push and PR
   - Multi-platform Rust tests (Linux, macOS, Windows)
   - Tests with stable and beta Rust
   - Feature combination testing
   - Code coverage reporting

2. **python-test.yml**: Python binding validation
   - Tests across Python 3.8-3.12
   - Wheel building verification
   - Import and functionality tests

3. **quality.yml**: Code quality checks
   - Clippy linting
   - Format checking
   - Documentation building
   - Security audit
   - Dependency checks

4. **release.yml**: Enhanced validation for releases
   - Version consistency checks
   - Full test suite on all platforms
   - Build artifacts for distribution
   - Python wheel building for all versions

## Making Contributions

1. **Fork the repository** and create your branch from `main`
2. **Write tests** for any new functionality
3. **Update documentation** as needed
4. **Run local CI checks** before pushing:
   ```bash
   cargo test --all-features
   cargo clippy --all-features -- -D warnings
   cargo fmt --check
   cargo doc --no-deps --all-features
   ```
5. **Create a Pull Request** with a clear description

## Code Style

- Follow Rust standard naming conventions
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Document public APIs with doc comments
- Keep commits focused and atomic

## Testing Guidelines

- Write unit tests for new functionality
- Integration tests go in `tests/` directory
- Use feature flags appropriately for conditional compilation
- Test error cases, not just happy paths

## Memory Considerations

On Windows or memory-constrained systems, you may need to use single-threaded builds:
```bash
cargo test --all-features -j 1
```

## Questions?

Feel free to open an issue for any questions about contributing!
