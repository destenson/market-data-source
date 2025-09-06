# PRP: Generator Configuration Structure

## Objective
Create a configuration structure that holds all parameters for market data generation including price, trend, and volatility settings.

## Context
Users need to configure how market data is generated. Configuration should be intuitive, with sensible defaults, and support the builder pattern for easy modification. This will be the primary interface for users to control data generation.

## Success Criteria
- Config struct with all basic parameters
- Default implementation with sensible values
- Builder pattern for configuration
- Validation of parameter ranges
- Clear documentation of each parameter

## Implementation Tasks
1. Create src/config.rs module
2. Define GeneratorConfig struct
3. Implement Default trait with sensible defaults
4. Create ConfigBuilder for fluent API
5. Add validation methods for parameters
6. Document parameter ranges and effects
7. Write tests for config validation

## Configuration Parameters
- Starting price (f64)
- Price bounds (min/max)
- Trend direction (enum: Bullish, Bearish, Sideways)
- Trend strength (percentage per period)
- Volatility (standard deviation)
- Time interval (using TimeInterval from PRP 02)
- Number of data points to generate
- Random seed (optional, for reproducibility)

## Validation Rules
- Starting price must be positive
- Min price must be less than max price
- Volatility must be non-negative
- Trend strength reasonable range (-100% to +100%)
- Number of points must be positive

## Validation Gates
```bash
# Build and test
cargo build --lib
cargo test config

# Check builder pattern works
cargo test config_builder

# Verify documentation
cargo doc --no-deps
```

## References
- Builder pattern in Rust: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
- Configuration best practices: https://rust-lang.github.io/api-guidelines/type-safety.html

## Dependencies
- PRP 01 (library structure)
- PRP 02 (TimeInterval type)

## Notes
- Use f64::INFINITY for unbounded prices
- Consider adding preset configurations (volatile, stable, trending)
- Validation should return Result types with clear error messages
- Builder should consume self for method chaining

## Confidence Score: 8/10
Standard configuration pattern with builder, common in Rust libraries.