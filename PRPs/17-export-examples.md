# PRP: Create Export Examples

## Objective
Create comprehensive examples demonstrating all export functionality (CSV, JSON, CouchDB, PNG) to help users quickly understand and use the export features.

## Context
After implementing export modules (PRPs 11-16), users need clear examples showing how to use each export format. Examples should cover common use cases and demonstrate best practices.

## Success Criteria
- Example for each export format
- Examples compile and run successfully
- Clear comments explaining each step
- Error handling demonstrated
- Output files can be verified
- README updated with example references

## Implementation Tasks
1. Create examples/export_csv.rs
2. Create examples/export_json.rs  
3. Create examples/export_couchdb.rs
4. Create examples/export_charts.rs
5. Create examples/export_all.rs (combined example)
6. Add example that exports to multiple formats
7. Update README.md with export examples
8. Add comments explaining each operation

## Example Structure
Each example should:
- Generate sample data
- Configure export options
- Perform the export
- Handle errors gracefully
- Print success message with file location

## Files to Create
```
examples/
  export_csv.rs      # CSV export example
  export_json.rs     # JSON export example
  export_couchdb.rs  # CouchDB upload example
  export_charts.rs   # PNG chart generation
  export_all.rs      # All formats at once
```

## Validation Gates
```bash
# Run each example
cargo run --example export_csv
cargo run --example export_json
cargo run --example export_charts
cargo run --example export_all

# Verify output files exist
# Check examples compile with --all-features
cargo build --examples --all-features
```

## Notes
- Examples should be self-contained
- Include sample output in comments
- Show both simple and advanced usage
- Demonstrate error handling
- Include performance considerations for large datasets
- Make CouchDB example optional (requires server)

## Dependencies
Requires completion of PRPs 11-16

## References
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Cargo examples: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples

## Confidence Score: 10/10
Straightforward example creation after export functionality is implemented.