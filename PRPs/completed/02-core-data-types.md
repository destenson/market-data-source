# PRP: Define Core Data Types

## Objective
Create the fundamental data structures for representing market data including OHLC candles, ticks, and timestamps.

## Context
Market data has standard representations. OHLC (Open, High, Low, Close) is the most common for candle/bar data. Need to support different time granularities and include volume. These types will be used throughout the library.

## Success Criteria
- Data types compile without warnings
- Types implement Debug, Clone, PartialEq
- Types are serializable (prepare for serde)
- Documentation explains each field
- Unit tests validate type construction

## Implementation Tasks
1. Create src/types.rs module
2. Define OHLC struct with price fields
3. Define Tick struct for tick-level data
4. Define Volume struct for volume data
5. Define TimeInterval enum for periods
6. Add derive macros for common traits
7. Add builder methods or constructors
8. Write unit tests for type validation

## Type Requirements
- OHLC: open, high, low, close prices (f64), volume (u64), timestamp
- Tick: price (f64), volume (u64), timestamp, bid/ask optionally
- TimeInterval: variants for 1min, 5min, 15min, 30min, 1hr, 4hr, 1day
- All types should handle NaN and Inf gracefully

## Validation Gates
```bash
# Compile and test
cargo build --lib
cargo test types

# Check documentation
cargo doc --no-deps --open

# Verify no clippy warnings
cargo clippy -- -D warnings
```

## References
- Standard financial data structures: https://www.investopedia.com/terms/o/ohlc.asp
- Rust f64 handling: https://doc.rust-lang.org/std/primitive.f64.html
- Consider chrono for timestamps: https://docs.rs/chrono/latest/chrono/

## Dependencies
- PRP 01 (library structure must exist)

## Notes
- Use f64 for prices (standard in financial applications)
- Use u64 for volume (no fractional shares in basic implementation)
- Consider using decimal types later for precision
- Timestamp can start as i64 (unix timestamp) or use chrono::DateTime

## Confidence Score: 8/10
Standard data types with well-known patterns in financial software.