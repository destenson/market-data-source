# PRP: Unit Tests for Core Components

## Objective
Create comprehensive unit tests for all core components to ensure correctness and prevent regressions.

## Context
Testing is critical for a data generation library. Tests should verify mathematical properties, edge cases, and ensure generated data meets specifications. This PRP focuses on unit tests for individual components.

## Success Criteria
- All public APIs have tests
- Edge cases are covered
- Statistical properties are verified
- Tests are deterministic (use seeds)
- Test coverage > 80%

## Implementation Tasks
1. Add test modules to each source file
2. Test OHLC construction and validation
3. Test config validation and builders
4. Test generator determinism
5. Test random walk statistics
6. Test volume correlation
7. Test timestamp generation
8. Add property-based tests

## Test Categories
- **Correctness**: Output matches specification
- **Validation**: Invalid inputs are rejected
- **Determinism**: Same seed produces same output
- **Statistics**: Distributions match parameters
- **Edge Cases**: Boundaries, zeros, limits

## Statistical Tests
- Mean/variance of generated prices
- Trend direction verification
- Volatility measurement
- Volume distribution shape
- Correlation coefficients

## Validation Gates
```bash
# Run all tests
cargo test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# Run specific test categories
cargo test unit::
cargo test integration::
```

## References
- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html
- Property testing: https://docs.rs/proptest/latest/proptest/
- Statistical tests: https://docs.rs/statistical/latest/statistical/

## Dependencies
- All component PRPs must be implemented
- Consider adding `proptest = "1.0"` for property tests
- Consider `approx = "0.5"` for float comparisons

## Notes
- Use `#[cfg(test)]` for test modules
- Use assert_eq! with custom messages
- Test both success and failure paths
- Consider doctests for examples

## Confidence Score: 8/10
Standard testing practices with focus on statistical validation.