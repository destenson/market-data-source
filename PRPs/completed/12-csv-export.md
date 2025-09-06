# PRP: Implement CSV Export Functionality

## Objective
Add CSV export capability for OHLC and Tick data, allowing users to save generated market data to CSV files for analysis in Excel, pandas, or other tools.

## Context
With serde serialization in place (PRP-11), we can now add CSV export. CSV is the most common format for financial data analysis. The csv crate provides efficient CSV writing with serde integration.

## Success Criteria
- Export Vec<OHLC> to CSV file with headers
- Export Vec<Tick> to CSV file with headers
- Handle file I/O errors gracefully
- Configurable CSV options (delimiter, quote style)
- Streaming write for large datasets
- Tests verify CSV output format

## Implementation Tasks
1. Add csv dependency to Cargo.toml
2. Create new module src/export/mod.rs
3. Create src/export/csv.rs with CsvExporter struct
4. Implement write_ohlc_to_csv function
5. Implement write_ticks_to_csv function
6. Add CSV writer options (headers, delimiter)
7. Implement streaming CSV writer for large datasets
8. Add convenience methods to MarketDataGenerator
9. Write integration tests with file verification

## Dependencies
- csv = "1.3"
- Requires PRP-11 (serde serialization) completed first

## References
- csv crate documentation: https://docs.rs/csv/latest/csv/
- CSV writer example: https://docs.rs/csv/latest/csv/struct.Writer.html
- Serde with CSV: https://docs.rs/csv/latest/csv/tutorial/index.html#writing-csv
- Error handling: https://docs.rs/csv/latest/csv/struct.Error.html

## File Structure
```
src/
  export/
    mod.rs       # Public export API
    csv.rs       # CSV implementation
```

## Validation Gates
```bash
# Build with CSV support
cargo build --lib

# Run CSV export tests
cargo test csv_export
cargo test export

# Verify generated CSV files are valid
# Tests should create and validate actual CSV files
```

## Notes
- Use csv::Writer with serde for automatic serialization
- Include headers by default (make configurable)
- Handle filesystem errors with proper Result types
- Consider buffered writing for performance
- Default to comma delimiter, make configurable
- Include example showing Excel compatibility

## Confidence Score: 9/10
Well-established pattern with the csv crate, straightforward implementation with serde.