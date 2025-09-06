# PRP: Market Data Generator Core Structure

## Objective
Implement the main MarketDataGenerator struct that orchestrates data generation using configuration and random number generation.

## Context
This is the primary struct users will interact with to generate market data. It should be easy to use with sensible defaults while allowing full customization. The generator will use the config from PRP 03 and produce data types from PRP 02.

## Success Criteria
- Generator struct with new() and with_config() constructors
- Can generate a single OHLC candle
- Can generate multiple candles
- Deterministic with seed
- Thread-safe if possible

## Implementation Tasks
1. Create src/generator.rs module
2. Define MarketDataGenerator struct
3. Add RNG field (random number generator)
4. Implement new() with default config
5. Implement with_config() for custom config
6. Add generate_candle() method
7. Add generate_series() method for multiple candles
8. Write comprehensive tests

## Generator Methods
- new() -> Self (uses default config)
- with_config(config: GeneratorConfig) -> Self
- generate_candle(&mut self) -> OHLC
- generate_series(&mut self, count: usize) -> Vec<OHLC>
- set_seed(&mut self, seed: u64) (for reproducibility)
- reset(&mut self) (restart generation)

## Internal State
- Current price (tracks last close)
- RNG instance (use rand crate)
- Configuration
- Timestamp tracker

## Validation Gates
```bash
# Build and test
cargo build --lib
cargo test generator

# Test deterministic behavior
cargo test test_deterministic

# Check thread safety
cargo test --features concurrent
```

## References
- rand crate for RNG: https://docs.rs/rand/latest/rand/
- StdRng for reproducible generation: https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html

## Dependencies
- PRP 01 (library structure)
- PRP 02 (OHLC type)
- PRP 03 (GeneratorConfig)
- Add `rand = "0.8"` to Cargo.toml

## Notes
- Use StdRng for reproducible random numbers
- Consider using SmallRng for performance
- Each candle's high must be >= max(open, close)
- Each candle's low must be <= min(open, close)

## Confidence Score: 7/10
Core functionality with RNG integration, needs careful testing.