# PRP: Implement JSON Export Functionality

## Objective
Add JSON export capability for market data, enabling integration with web applications, APIs, and NoSQL databases. Support both single-file and streaming JSON Lines format.

## Context
With serde serialization (PRP-11) complete, JSON export is straightforward. JSON is essential for web APIs and document databases. We'll support both standard JSON arrays and JSON Lines (newline-delimited JSON) for streaming.

## Success Criteria
- Export Vec<OHLC> to JSON file
- Export Vec<Tick> to JSON file  
- Support JSON Lines format for streaming
- Pretty-print option for human readability
- Handle file I/O errors gracefully
- Tests verify JSON structure and validity

## Implementation Tasks
1. Create src/export/json.rs module
2. Implement write_ohlc_to_json function
3. Implement write_ticks_to_json function
4. Add JSON Lines writer for streaming
5. Implement pretty-print option
6. Add compression option (gzip)
7. Create JsonExportOptions struct
8. Add convenience methods to generator
9. Write tests validating JSON structure

## Dependencies
- serde_json = "1.0" (already added in PRP-11)
- flate2 = "1.0" (optional, for gzip compression)

## References
- serde_json Writer: https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html
- JSON Lines format: https://jsonlines.org/
- Pretty printing: https://docs.rs/serde_json/latest/serde_json/fn.to_string_pretty.html
- Streaming serialization: https://docs.rs/serde_json/latest/serde_json/struct.StreamSerializer.html

## Validation Gates
```bash
# Build and test JSON export
cargo build --lib
cargo test json_export

# Verify JSON validity
# Tests should parse exported JSON and validate structure
cargo test json_roundtrip
```

## Notes
- Use serde_json::to_writer for efficiency
- JSON Lines format: one JSON object per line
- Pretty-print adds indentation but increases file size
- Consider memory usage for large datasets
- Support both array format and streaming format
- Include example of JavaScript/Python consuming the JSON

## Confidence Score: 9/10
Direct serialization with serde_json, standard patterns.