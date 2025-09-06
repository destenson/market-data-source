# PRP: Create Basic Usage Example

## Objective
Create a working example that demonstrates basic library usage, matching the README's promised API.

## Context
The README shows example usage with MarketData::new() and fetch methods. While we're focusing on generation first, we need an example that shows how to use the generator. This will also serve as an integration test.

## Success Criteria
- Example compiles and runs
- Generates visible output
- Demonstrates key features
- Can be run with `cargo run --example basic`
- Matches README's promised simplicity

## Implementation Tasks
1. Create examples/basic.rs
2. Import library types
3. Show default generation
4. Show configured generation
5. Display generated data
6. Add helpful comments
7. Test example execution

## Example Components
- Create generator with defaults
- Generate 10 OHLC candles
- Print in readable format
- Show configuration options
- Demonstrate reproducible generation with seed

## Output Format
- Pretty-print OHLC data
- Show timestamp, OHLC, volume
- Include summary statistics
- Make output educational

## Validation Gates
```bash
# Run example
cargo run --example basic

# Check output
cargo run --example basic | head -20

# Verify no panics
cargo run --example basic 2>&1 | grep -v panic
```

## References
- Rust examples: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples
- Pretty printing: https://doc.rust-lang.org/std/fmt/

## Dependencies
- All previous PRPs (needs working library)

## Notes
- Keep example simple and focused
- Add comments explaining each step
- Consider adding more examples later
- Output should be self-explanatory

## Confidence Score: 8/10
Straightforward example once core functionality exists.