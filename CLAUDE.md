# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CRITICAL REMINDERS - ALWAYS FOLLOW

1. **NO EMOJIS EVER** - Do not use emojis in code, comments, output, or documentation
2. **USE MCP TOOLS** - Always use the Cargo MCP tools (mcp__Cargo__*) instead of bash commands for Rust operations
3. **MEMORY CONSTRAINT** - Always use `-j 1` flag when building (limits to single job due to 8GB RAM constraint)
4. **NO DIRECTORY CHANGES** - Use absolute paths instead of changing directories
5. **CHECK THIS FILE FIRST** - Always read and follow these instructions

## Build & Development Commands

### Build the project
USE MCP TOOL: `mcp__Cargo__cargo_build` with args `["-j", "1"]`
```bash
cargo build -j 1  # Limited to 1 job due to memory constraints
```

### Run the project
USE MCP TOOL: `mcp__Cargo__cargo_run`
```bash
cargo run
```

### Run tests
USE MCP TOOL: `mcp__Cargo__cargo_test` with args `["--all-features", "-j", "1"]`
```bash
cargo test --all-features -j 1  # Limited to 1 job due to memory constraints
```

### Run a specific test
USE MCP TOOL: `mcp__Cargo__cargo_test` with args `["test_name", "-j", "1"]`
```bash
cargo test test_name -j 1  # Limited to 1 job due to memory constraints
```

### Check for compilation errors without building
USE MCP TOOL: `mcp__Cargo__cargo_check`
```bash
cargo check
```

### Format code
USE MCP TOOL: `mcp__Cargo__cargo_fmt`
```bash
cargo fmt
```

### Run linter
USE MCP TOOL: `mcp__Cargo__cargo_clippy`
```bash
cargo clippy
```

**ALWAYS use the Cargo MCP tools instead of bash commands!**

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
- Current version is 0.3.0 using Rust edition 2024
- The public API will expose a `MarketData` struct with methods for fetching real-time and historical data
- Synthetic data generation is a planned feature for simulating market conditions

## Version Synchronization

When updating versions for a new release:
1. Update version in `Cargo.toml` (currently 0.3.0)
2. Update version in `pyproject.toml` to match (currently 0.3.0)
3. Update version in README.md Rust installation example
4. Run `maturin build --release -j 1` to build Python wheels
5. The version will automatically be available in Python via `market_data_source.__version__`
