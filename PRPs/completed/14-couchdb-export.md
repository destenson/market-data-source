# PRP: Implement CouchDB Export Functionality

## Objective
Add CouchDB integration to directly export market data to CouchDB databases, enabling NoSQL storage and replication of generated market data for analysis and backtesting.

## Context
CouchDB is a document database that stores JSON. With serde serialization (PRP-11) and JSON export (PRP-13) in place, we can add direct CouchDB integration. This enables persistent storage and querying of generated data.

## Success Criteria
- Connect to CouchDB instance
- Create/select database for market data
- Bulk insert OHLC documents
- Bulk insert Tick documents
- Add timestamp-based views for querying
- Handle connection errors gracefully
- Tests with mock CouchDB responses

## Implementation Tasks
1. Add couch_rs dependency to Cargo.toml
2. Create src/export/couchdb.rs module
3. Implement CouchDbExporter struct with connection
4. Create database if not exists
5. Implement bulk document insert for OHLC
6. Implement bulk document insert for Ticks
7. Add design documents with views
8. Implement error handling and retry logic
9. Write integration tests with mock server

## Dependencies
- couch_rs = "0.10" or latest
- reqwest for HTTP client (dependency of couch_rs)
- tokio for async runtime (if using async)

## References
- couch_rs documentation: https://docs.rs/couch_rs/latest/couch_rs/
- CouchDB bulk docs API: https://docs.couchdb.org/en/stable/api/database/bulk-api.html
- CouchDB design documents: https://docs.couchdb.org/en/stable/ddocs/index.html
- Error handling: https://docs.rs/couch_rs/latest/couch_rs/error/enum.CouchError.html

## CouchDB Document Structure
Documents will include:
- _id: Generated from symbol + timestamp
- type: "ohlc" or "tick"
- symbol: String
- timestamp: Unix timestamp
- data: OHLC or Tick fields

## Validation Gates
```bash
# Build with CouchDB support
cargo build --lib

# Run CouchDB tests (requires mock or test instance)
cargo test couchdb

# Integration test with real CouchDB (optional)
# docker run -d -p 5984:5984 couchdb:latest
cargo test --ignored couchdb_integration
```

## Notes
- Use bulk insert API for performance
- Consider async implementation for better throughput
- Add connection pooling for production use
- Include view for querying by timestamp range
- Document structure should be indexed efficiently
- Handle CouchDB revision conflicts appropriately
- Consider making this an optional feature flag

## Confidence Score: 7/10
External dependency with network I/O, requires careful error handling and testing strategy.