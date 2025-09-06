# PRP: Create Integration Tests for Export Functionality

## Objective
Create comprehensive integration tests that verify all export functionality works correctly end-to-end, including file I/O, data integrity, and format validation.

## Context
After implementing export modules, we need integration tests that verify the complete export pipeline. Tests should generate data, export it, and validate the output files are correct and usable.

## Success Criteria
- Integration tests for each export format
- Tests verify file creation and content
- Round-trip tests (export then import)
- Performance benchmarks for large datasets
- Tests handle cleanup of test files
- All tests pass in CI environment

## Implementation Tasks
1. Create tests/export_integration.rs
2. Add test for CSV export with validation
3. Add test for JSON export with parsing
4. Add test for JSON Lines streaming
5. Add test for PNG chart generation
6. Add round-trip test (export then re-import)
7. Add performance test with large dataset
8. Implement test file cleanup
9. Add test for export error conditions

## Test Structure
```
tests/
  export_integration.rs  # All export integration tests
  fixtures/             # Test data if needed
    expected_output.csv
    expected_output.json
```

## Test Categories
- Correctness: Data is exported accurately
- Format: Output matches expected format
- Performance: Large datasets handled efficiently
- Error handling: Failures handled gracefully
- Cleanup: Temporary files removed

## Validation Gates
```bash
# Run integration tests
cargo test --test export_integration

# Run with all features
cargo test --all-features

# Run benchmarks
cargo bench export

# Check test coverage
cargo tarpaulin --out Html
```

## Notes
- Use tempfile crate for test file management
- Verify CSV headers and data rows
- Parse JSON to verify structure
- Check PNG file headers for validity
- Include edge cases (empty data, special characters)
- Test with different configuration options
- Clean up test files in test teardown

## Dependencies
- tempfile = "3.8" for test file management
- Additional dev-dependencies for validation

## References
- Integration testing: https://doc.rust-lang.org/book/ch11-03-test-organization.html
- tempfile crate: https://docs.rs/tempfile/latest/tempfile/
- CSV validation: https://docs.rs/csv/latest/csv/#example-reading-csv
- JSON schema validation: https://docs.rs/jsonschema/latest/jsonschema/

## Confidence Score: 9/10
Standard integration testing patterns with file I/O validation.