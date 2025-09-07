# PRP-21: Pre-Publication Code Quality Fixes

## Context & Motivation

Before publishing to crates.io and PyPI, the codebase must meet quality standards expected by the Rust and Python communities. Current issues blocking publication:

- **34 clippy warnings** identified via `cargo clippy --all-targets --all-features -- -D`
- **WebSocket functionality** failing (13/15 server tests passing - 86.7%)
- **Deprecated code** in use that may cause future compatibility issues

**Priority**: CRITICAL - These are blockers for publication acceptance and community adoption.

## Requirements

### Must Fix Before Publication
1. **Clippy warnings resolution** (34 warnings)
   - 27 unwrap() calls in src/config.rs (Decimal::from_f64 conversions)
   - 7 format string efficiency warnings
   - Missing Default trait for ConfigBuilder

2. **WebSocket endpoint repair** 
   - Current test failure pattern: WebSocket connection test fails in test-server.ps1
   - Location: src/server/websocket.rs and related WebSocket handling

3. **Deprecated code cleanup**
   - generate_candle() usage at src/generator.rs:201 (use generate_ohlc())
   - PyO3 IntoPy trait migration to IntoPyObject
   - 4 deprecated ExportError variants review

## Implementation Blueprint

### Phase 1: Clippy Compliance
1. Address unwrap() calls in src/config.rs by replacing with expect() calls with descriptive messages
2. Update format! macro calls to use direct variable interpolation
3. Implement Default trait for ConfigBuilder

### Phase 2: WebSocket Functionality
1. Investigate WebSocket upgrade handling in HTTP to WS transition
2. Review WebSocket route registration and handler implementation
3. Add proper error handling for WebSocket connection lifecycle

### Phase 3: Deprecated Code Cleanup  
1. Replace generate_candle() calls with generate_ohlc()
2. Migrate PyO3 trait usage to current recommended patterns
3. Review and update ExportError enum variants

## Success Criteria

### Validation Gates
```bash
# All clippy warnings resolved
cargo clippy --all-targets --all-features -- -D warnings

# All tests passing including WebSocket
cargo test --all-features

# Server tests at 100% pass rate
# Run existing server test scripts to verify WebSocket functionality
```

### Verification Steps
- `cargo clippy` produces zero warnings
- `cargo test --all-features` shows 100% pass rate
- Server test suite shows 15/15 tests passing (up from 13/15)
- No deprecated warnings in build output

## Dependencies & References

**Existing Patterns**: 
- Reference src/types.rs for proper error handling patterns
- Review src/export/ modules for Result<> usage examples

**Documentation**: 
- Clippy lint descriptions: https://rust-lang.github.io/rust-clippy/master/
- PyO3 migration guide: https://pyo3.rs/v0.23.0/migration.html

**Testing Approach**:
- Use existing test patterns in tests/ directory
- Verify against current TODO.md quality metrics

## Implementation Tasks

1. Run `cargo clippy --fix --allow-dirty` for auto-fixable warnings
2. Manually address remaining unwrap() calls with proper error handling
3. Implement Default trait for ConfigBuilder struct  
4. Debug WebSocket endpoint using browser developer tools or WebSocket client
5. Update generate_candle() call to generate_ohlc() in src/generator.rs
6. Research and implement PyO3 IntoPyObject migration
7. Review ExportError deprecated variants and update callers
8. Run full test suite validation
9. Verify server functionality with existing test scripts

## Estimated Effort
**2-3 days** (focused implementation with testing)

## Risk Mitigation
- Test each fix incrementally to avoid breaking working functionality
- Maintain backward compatibility where possible during deprecated code cleanup
- Use expect() with descriptive messages rather than unwrap() for clear failure points

## Success Score
**8/10** - High confidence due to clear error messages from tooling and existing patterns to follow.