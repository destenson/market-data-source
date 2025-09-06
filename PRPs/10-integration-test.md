# PRP: Integration Test Suite

## Objective
Create integration tests that verify the entire library works together correctly from a user's perspective.

## Context
While unit tests verify individual components, integration tests ensure everything works together. These tests should mirror real usage patterns and verify end-to-end functionality.

## Success Criteria
- Tests cover common usage patterns
- Full generation pipeline is tested
- Performance benchmarks included
- Tests are in tests/ directory
- Can run with `cargo test --test '*'`

## Implementation Tasks
1. Create tests/generation.rs
2. Test default generation flow
3. Test custom configuration flow
4. Test large data generation
5. Test edge configurations
6. Add performance benchmarks
7. Test error scenarios

## Integration Scenarios
- Generate 1000 candles with defaults
- Generate with extreme volatility
- Generate with strong trends
- Generate with price bounds
- Generate different time intervals
- Test memory usage for large datasets

## Performance Tests
- Measure generation speed (candles/second)
- Memory usage for large series
- Configuration overhead
- Verify O(n) complexity

## Validation Gates
```bash
# Run integration tests
cargo test --test '*'

# Run benchmarks
cargo bench

# Check for memory leaks
cargo test --test memory_usage
```

## References
- Integration testing: https://doc.rust-lang.org/book/ch11-03-test-organization.html
- Benchmarking: https://docs.rs/criterion/latest/criterion/

## Dependencies
- All component PRPs implemented
- Consider `criterion = "0.5"` for benchmarks

## Notes
- Keep tests independent
- Use temporary files if needed
- Test realistic scenarios
- Include performance regression tests

## Confidence Score: 7/10
Comprehensive testing requiring all components to work together.