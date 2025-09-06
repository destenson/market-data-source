# PRP: Add Serde Serialization Support

## Objective
Add serde serialization/deserialization support to all core data types to enable data export and import functionality. This is the foundation for CSV, JSON, and other export formats.

## Context
Currently, the library has no serialization support. Adding serde with derive macros will enable all data types to be serialized to various formats. This PRP focuses solely on adding the serde dependency and derive macros to existing types.

## Success Criteria
- All core types can be serialized to JSON
- All core types can be deserialized from JSON
- Serialization preserves all data accurately
- Optional fields handled correctly
- Tests verify round-trip serialization

## Implementation Tasks
1. Add serde and serde_json dependencies to Cargo.toml
2. Add serde derive macros to OHLC struct in types.rs
3. Add serde derive macros to Tick struct in types.rs
4. Add serde derive macros to Volume struct in types.rs
5. Add serde derive macros to TimeInterval enum in types.rs
6. Add serde derive macros to GeneratorConfig in config.rs
7. Add serde derive macros to TrendDirection enum in config.rs
8. Write unit tests for JSON serialization round-trips

## Dependencies
- serde = "1.0" with derive feature
- serde_json = "1.0" for testing

## References
- Serde documentation: https://serde.rs/derive.html
- Serde with enums: https://serde.rs/enum-representations.html
- Custom serialization if needed: https://serde.rs/custom-serialization.html

## Validation Gates
```bash
# Build with new dependencies
cargo build --lib

# Run tests including new serialization tests
cargo test serde
cargo test serialization
cargo test json

# Check that all types implement Serialize/Deserialize
cargo doc --no-deps
```

## Notes
- Use #[derive(Serialize, Deserialize)] on all public types
- Consider using #[serde(rename_all = "camelCase")] for JavaScript compatibility
- Volume struct might need custom serialization since it wraps u64
- TimeInterval enum should use string representation for readability

## Confidence Score: 9/10
Simple addition of derive macros to existing types with well-documented library.