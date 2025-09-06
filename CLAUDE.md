# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

### Build the project
```bash
cargo build
```

### Build in release mode
```bash
cargo build --release
```

### Run the project
```bash
cargo run
```

### Run tests
```bash
cargo test
```

### Run a specific test
```bash
cargo test test_name
```

### Check for compilation errors without building
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Run linter
```bash
cargo clippy
```

## Architecture Overview

This is a Rust library for fetching and processing market data from various financial APIs. The codebase is currently in early development with a minimal structure:

- **Main entry point**: `src/main.rs` - Currently contains a basic Hello World implementation
- **Core purpose**: Provide a unified interface for accessing real-time and historical market data from multiple financial data sources
- **Key design principles**:
  - Extensible architecture for adding new data sources
  - Support for both real-time and historical data
  - Includes realistic data generation capabilities for testing trading strategies

## Development Notes

- The library is designed to be used as a dependency in other Rust projects
- Current version is 0.1.0 using Rust edition 2024
- The public API will expose a `MarketData` struct with methods for fetching real-time and historical data
- Synthetic data generation is a planned feature for simulating market conditions