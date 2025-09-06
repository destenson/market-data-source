# PRP: Timestamp and Time Series Generation

## Objective
Implement timestamp generation for market data that respects market hours, intervals, and creates realistic time series.

## Context
Market data needs accurate timestamps. Must handle different intervals (1min, 5min, daily), market hours (9:30 AM - 4:00 PM ET for US stocks), and weekends/holidays. Timestamps are crucial for realistic data.

## Success Criteria
- Generate sequential timestamps at configured intervals
- Skip weekends (Saturday, Sunday)
- Optional market hours enforcement
- Support multiple time zones
- Handle daylight saving time

## Implementation Tasks
1. Create src/time.rs module
2. Define MarketCalendar struct
3. Implement timestamp iterator
4. Add market hours validation
5. Create weekend detection
6. Add holiday support (basic US holidays)
7. Implement time zone handling
8. Write tests for edge cases

## Time Components
- Starting timestamp (configurable)
- Interval (from TimeInterval enum)
- Market hours (optional, e.g., 9:30-16:00)
- Time zone (default UTC)
- Weekend handling (skip or include)

## Market Hours Rules
- US Stock Market: 9:30 AM - 4:00 PM Eastern
- Forex: 24 hours (Sunday evening - Friday evening)
- Crypto: 24/7
- Configurable per market type

## Validation Gates
```bash
# Build and test
cargo test time

# Test weekend skipping
cargo test test_weekend_skip

# Test market hours
cargo test test_market_hours

# Verify DST handling
cargo test test_daylight_saving
```

## References
- chrono crate: https://docs.rs/chrono/latest/chrono/
- Market hours: https://www.nyse.com/markets/hours-calendars
- Time zone handling: https://docs.rs/chrono-tz/latest/chrono_tz/

## Dependencies
- PRP 02 (TimeInterval enum)
- Add `chrono = "0.4"` to Cargo.toml
- Consider `chrono-tz = "0.8"` for timezone support

## Notes
- Use chrono::DateTime<Utc> for consistency
- Iterator pattern for generating sequences
- Consider caching holiday calendars
- Make market hours optional for 24/7 markets

## Confidence Score: 6/10
Complex time handling with market-specific rules and edge cases.