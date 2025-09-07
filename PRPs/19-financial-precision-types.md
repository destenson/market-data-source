# PRP: Financial Precision Types

## Objective
Replace floating-point (`f64`) price representations with high-precision decimal types to ensure financial-grade accuracy in all price calculations and prevent precision errors critical to financial applications.

## Context
The current implementation uses `f64` for all price fields (OHLC, Tick), which causes precision errors in financial calculations. This is a **CRITICAL** issue for financial applications where even small rounding errors can compound into significant problems over time or with large datasets.

Financial applications require exact decimal arithmetic to:
- Prevent rounding errors in price calculations
- Ensure reproducible results across different systems
- Meet regulatory requirements for financial accuracy
- Support precise currency and trading calculations

## Success Criteria
- All price fields use `rust_decimal::Decimal` instead of `f64`
- No precision loss in financial calculations
- All existing tests pass with decimal types
- Export formats handle decimal serialization correctly
- Generator algorithms work seamlessly with decimal arithmetic
- Performance impact is minimal and acceptable

## Implementation Tasks
1. Add `rust_decimal` dependency to Cargo.toml
2. Update OHLC struct to use Decimal for price fields
3. Update Tick struct to use Decimal for price fields
4. Update GeneratorConfig to use Decimal for price parameters
5. Modify RandomWalkGenerator to use decimal arithmetic
6. Update export modules to handle Decimal serialization
7. Fix all compilation errors and type mismatches
8. Update tests to use Decimal types
9. Validate numerical accuracy improvements
10. Update documentation to reflect decimal precision

## Type Changes Required

### Core Data Types (src/types.rs)
```rust
use rust_decimal::Decimal;

// Before: f64 prices
// After: Decimal prices
pub struct OHLC {
    pub open: Decimal,     // was f64
    pub high: Decimal,     // was f64
    pub low: Decimal,      // was f64
    pub close: Decimal,    // was f64
    pub volume: u64,       // unchanged
    pub timestamp: i64,    // unchanged
}

pub struct Tick {
    pub price: Decimal,    // was f64
    pub volume: u64,       // unchanged
    pub timestamp: i64,    // unchanged
    pub bid: Option<Decimal>,  // was Option<f64>
    pub ask: Option<Decimal>,  // was Option<f64>
}
```

### Generator Configuration (src/config.rs)
```rust
pub struct GeneratorConfig {
    pub initial_price: Decimal,    // was f64
    pub min_price: Option<Decimal>, // was Option<f64>
    pub max_price: Option<Decimal>, // was Option<f64>
    pub drift: Decimal,            // was f64
    pub volatility: Decimal,       // was f64
    // ... other fields unchanged
}
```

### Algorithm Updates (src/algorithms/random_walk.rs)
- Convert all price calculations to use Decimal arithmetic
- Replace `f64` random number generation with Decimal-compatible approach
- Ensure drift and volatility calculations maintain precision

## Validation Gates
```bash
# Add dependency
cargo add rust_decimal --features serde

# Compile with new types
cargo build --all

# Run all tests to ensure compatibility
cargo test

# Check for clippy warnings
cargo clippy -- -D warnings

# Test export functionality
cargo run --example export_all

# Verify precision improvements
cargo test --test precision_test
```

## Dependencies
- rust_decimal crate (with serde feature for serialization)
- All existing PRPs (01-18) as this modifies core types

## Migration Strategy
1. **Phase 1**: Add dependency and update core types
2. **Phase 2**: Update generator logic and algorithms
3. **Phase 3**: Fix export modules and serialization
4. **Phase 4**: Update all tests and examples
5. **Phase 5**: Validate precision improvements

## Precision Benefits
- **Exact Decimal Arithmetic**: No floating-point rounding errors
- **Financial Compliance**: Meets standards for financial calculations
- **Reproducible Results**: Same calculations always produce identical results
- **Currency Support**: Proper handling of monetary amounts with fixed decimal places
- **Regulatory Compliance**: Meets audit requirements for financial accuracy

## Performance Considerations
- Decimal arithmetic is slower than `f64` but still acceptable for most use cases
- Memory usage increases slightly (16 bytes vs 8 bytes per Decimal)
- Serialization/deserialization may be marginally slower
- The accuracy benefits far outweigh the performance cost for financial applications

## Error Patterns & Solutions
```bash
# Common compilation errors after conversion:
error[E0277]: the trait bound `Decimal: From<f64>` is not satisfied
# Solution: Use Decimal::from_f64() or Decimal::new()

error[E0277]: cannot multiply `Decimal` by `f64`  
# Solution: Convert f64 to Decimal first: Decimal::from_f64(value).unwrap()

error[E0308]: mismatched types (expected `f64`, found `Decimal`)
# Solution: Update function signatures and use .to_f64() when needed
```

## Testing Strategy
1. **Precision Tests**: Compare calculations with known exact results
2. **Round-Trip Tests**: Ensure serialization preserves precision
3. **Generator Tests**: Verify algorithm output remains realistic
4. **Export Tests**: Confirm all formats handle Decimal correctly
5. **Performance Tests**: Measure impact on generation speed

## References
- rust_decimal crate: https://docs.rs/rust_decimal/latest/rust_decimal/
- Financial precision best practices: https://husobee.github.io/money/float/2016/09/23/never-use-floats-for-currency.html
- Decimal arithmetic: https://en.wikipedia.org/wiki/Decimal_floating_point

## Notes
- This is a **BREAKING CHANGE** requiring version bump to v0.3.0
- Consider providing migration utilities for existing users
- Export formats should maintain human-readable decimal representation
- Some random number generation may need adaptation for Decimal types
- Test with real-world financial scenarios to validate improvements

## Confidence Score: 9/10
Well-established pattern for financial applications. The rust_decimal crate is mature and widely used in financial Rust applications.