# TODO

## Current Implementation Status

### Recently Completed ✅
- **Compilation Fixes**: All critical build errors resolved (2025-01-07)
  - Fixed serde attribute conditional compilation in config.rs
  - Added WriteFailed variant to ExportError enum (marked deprecated by linter)
  - Implemented build_to_buffer method for ChartBuilder
  - Fixed CSV import paths
  - Fixed timestamp conversion methods
  - Removed unused imports and variables
- **REST/WebSocket Server**: Full API server with runtime discovery, control endpoint, and clean shutdown
- **PRP-20**: Python Bindings - Full PyO3 integration with examples and tests
- **19 PRPs Completed**: All foundational PRPs (01-13, 15-20) - Complete export infrastructure
- **Export Module**: Fully functional with trait-based design, proper error types, and unified architecture
- **Feature Flags**: Proper separation of optional dependencies
- **Version 0.2.0**: Bumped version reflecting financial precision improvements

### Active Issues ⚠️

#### Critical - Test Suite Failures
- **38 test compilation errors**: All test files using f64 literals instead of Decimal
  - Files affected: src/export/csv.rs, src/export/json.rs, src/export/couchdb.rs, src/export/chart.rs
  - Fix: Replace all float literals with `Decimal::from_f64()` or `Decimal::from_str()`
  - Example: `100.0` should be `Decimal::from_f64(100.0).unwrap()`

#### High Priority - Server Issues
- **WebSocket endpoint failing**: test-server.ps1 shows WebSocket check fails
  - Need to investigate WebSocket handler implementation
  - May need proper WebSocket upgrade handling
- **Uptime tracking not implemented**: src/server/routes.rs:101
  - Currently returns "not tracked" placeholder
  - Should track actual server start time

#### Medium Priority - Code Quality
- **186 unwrap() calls**: Technical debt across 9 files
  - Highest concentration: src/types.rs (52), src/config.rs (44)
  - Should replace with proper error handling
- **Deprecated methods still in use**:
  - `generate_candle()` used in src/generator.rs:201 (should use `generate_ohlc()`)
  - PyO3 `IntoPy` trait deprecated (migration needed to `IntoPyObject`)
  
### Code Quality Metrics
- **No TODO/FIXME comments** found in active codebase (exceptionally clean)
- **Deprecated error variants**: 4 variants in ExportError marked deprecated by external process
- **Unused variables**: `_lower` in chart.rs:221, `_config` instances in tests
- **Environment config limitation**: src/env.rs:157 - "For now, most variables are optional"

## 🎯 Immediate Priorities

### Critical - Fix Test Suite (1-2 days)
1. [ ] **Fix all test Decimal conversions** - 38 compilation errors
   ```rust
   // Before: OHLC::new(100.0, 105.0, 99.0, 103.0, 1000, 1234567890000)
   // After: OHLC::new(
   //     Decimal::from_f64(100.0).unwrap(),
   //     Decimal::from_f64(105.0).unwrap(),
   //     Decimal::from_f64(99.0).unwrap(),
   //     Decimal::from_f64(103.0).unwrap(),
   //     1000,
   //     1234567890000
   // )
   ```

### High Priority - Server Fixes (2-3 days)
1. [ ] **Fix WebSocket endpoint**
   - Investigate WebSocket upgrade handling in src/server/websocket.rs
   - Ensure proper connection upgrade from HTTP to WS
   - Add integration tests for WebSocket streaming

2. [ ] **Implement uptime tracking**
   - Add server start time to AppState
   - Calculate and return actual uptime in status endpoint
   - Location: src/server/routes.rs:101

### Medium Priority - Error Handling (1 week)
1. [ ] **Replace unwrap() calls with proper Result handling**
   - Priority files by usage count:
     - src/types.rs: 52 occurrences
     - src/config.rs: 44 occurrences  
     - src/export/json.rs: 31 occurrences
     - src/python.rs: 15 occurrences
     - src/generator.rs: 13 occurrences
     - src/algorithms/random_walk.rs: 15 occurrences

2. [ ] **Clean up deprecated code**
   - Replace `generate_candle()` with `generate_ohlc()`
   - Migrate PyO3 from `IntoPy` to `IntoPyObject`
   - Review deprecated ExportError variants

### Lower Priority - Enhancements
1. [ ] **Expand environment configuration** (src/env.rs:157)
   - Make more configuration options available via environment variables
   - Add validation for environment variable values
   
2. [ ] **CouchDB dependency issue** (PRP-14)
   - Currently blocked by SIMD feature conflict
   - Update to couch_rs 0.10+ when possible

## 🚀 Next Major Features

### Enhanced Market Realism
- [ ] **Volatility Clustering**: GARCH/EGARCH models
- [ ] **Mean Reversion**: Ornstein-Uhlenbeck process
- [ ] **Jump Diffusion**: Sudden price jumps for news events
- [ ] **Intraday Patterns**: Market hours, opening volatility, lunch lull

### Market Microstructure
- [ ] **Order Book Simulation**: Depth, liquidity, market impact
- [ ] **Dynamic Spreads**: Spread that widens during volatility
- [ ] **Realistic Volume Profiles**: U-shaped intraday volume
- [ ] **Price-Volume Correlation**: Higher volume on large moves

### Server Enhancements
- [ ] **Authentication**: Add auth middleware for control endpoint
- [ ] **Rate Limiting**: Implement actual rate limiting (currently just configured)
- [ ] **Metrics Endpoint**: Add Prometheus-compatible metrics
- [ ] **Config Hot Reload**: Implement configuration reload without restart
- [ ] **Multi-Protocol Support**: Add gRPC alongside REST/WebSocket

### Data Export & Integration
- [ ] **Parquet Support**: Efficient columnar storage
- [ ] **Excel Export**: XLSX format support
- [ ] **SQLite Export**: Embedded database support
- [ ] **Real Data Sources**: Yahoo Finance, Alpha Vantage, IEX Cloud integrations

## 🎯 KILLER FEATURES - Level 2 & Options

### Order Book (Level 2) Data
- [ ] **Full Order Book**: Multiple price levels with bid/ask sizes
- [ ] **Order Flow Dynamics**: Realistic order placement/cancellation patterns
- [ ] **Market Depth Evolution**: How order book changes over time
- [ ] **HFT Patterns**: High-frequency trading signatures

### Options Pricing & Greeks
- [ ] **Black-Scholes Engine**: Standard options pricing model
- [ ] **Implied Volatility Surface**: Realistic IV skew and term structure
- [ ] **Greeks Calculation**: Delta, Gamma, Theta, Vega, Rho
- [ ] **Options Chains**: Full strikes and expirations

## 🧪 Testing & Validation

### Statistical Tests Needed
- [ ] **Jarque-Bera Test**: Validate distribution normality
- [ ] **Augmented Dickey-Fuller**: Test for stationarity
- [ ] **ARCH Test**: Validate heteroskedasticity
- [ ] **Hurst Exponent**: Measure long-term memory

### Performance Benchmarks
- [ ] **Generation Speed**: Benchmark data points per second
- [ ] **Memory Usage**: Profile memory consumption
- [ ] **Comparison Suite**: Compare with real market data statistics

## 📚 Documentation

### Priority Documentation
- [ ] **API Reference**: Complete rustdoc documentation
- [ ] **Server API Guide**: REST/WebSocket endpoint documentation
- [ ] **Migration Guide**: For users updating from v0.1 to v0.2
- [ ] **Cookbook**: Common scenarios (crashes, rallies, ranging markets)

## 🔧 Technical Debt Summary

| Category | Count | Priority | Estimated Effort |
|----------|-------|----------|------------------|
| Test compilation errors | 38 | CRITICAL | 1-2 days |
| unwrap() calls | 186 | High | 1 week |
| WebSocket functionality | 1 | High | 2-3 days |
| Deprecated code | 5+ | Medium | 3 days |
| Missing features | 10+ | Low | Ongoing |

## 📝 Notes

- Focus on test suite restoration as highest priority
- WebSocket functionality critical for real-time streaming use cases
- Error handling improvements will significantly improve stability
- Consider creating helper functions for Decimal conversions in tests
- The codebase is remarkably clean with no TODO/FIXME comments
- Server test suite shows 13/15 tests passing (86.7% pass rate)

## 🔥 Recent Progress

The project has been successfully restored from a completely broken state:
- ✅ Library builds successfully with all features
- ✅ Server runs and handles most API requests
- ✅ 13/15 server tests passing
- ⚠️ Test suite needs Decimal type fixes
- ⚠️ WebSocket endpoint needs investigation

Next sprint should focus on:
1. Fixing test suite (Decimal conversions)
2. Fixing WebSocket endpoint
3. Reducing unwrap() usage in critical paths