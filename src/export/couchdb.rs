//! CouchDB export functionality for market data
//!
//! This module provides functionality to export market data directly to CouchDB,
//! a NoSQL document database that stores JSON documents.

use crate::export::{DataExporter, ExportResult};
use crate::types::{OHLC, Tick};
use couch_rs::{Client, database::Database, document::TypedCouchDocument, error::CouchError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::borrow::Cow;

/// CouchDB exporter for market data
pub struct CouchDbExporter {
    /// CouchDB server URL
    server_url: String,
    /// Database name for market data
    database_name: String,
    /// Username for authentication (optional)
    username: Option<String>,
    /// Password for authentication (optional)
    password: Option<String>,
    /// Batch size for bulk operations
    batch_size: usize,
}

impl CouchDbExporter {
    /// Create a new CouchDB exporter with default settings
    pub fn new(server_url: impl Into<String>, database_name: impl Into<String>) -> Self {
        Self {
            server_url: server_url.into(),
            database_name: database_name.into(),
            username: None,
            password: None,
            batch_size: 1000,
        }
    }

    /// Set authentication credentials
    pub fn with_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set batch size for bulk operations
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Connect to CouchDB and get database handle
    async fn get_database(&self) -> Result<Database, CouchError> {
        let client = if let (Some(username), Some(password)) = (&self.username, &self.password) {
            Client::new(&self.server_url, username, password)?
        } else {
            Client::new_no_auth(&self.server_url)?
        };

        // Create database if it doesn't exist
        let db = match client.db(&self.database_name).await {
            Ok(db) => db,
            Err(_) => {
                client.make_db(&self.database_name).await?;
                client.db(&self.database_name).await?
            }
        };

        Ok(db)
    }

    /// Export OHLC data to CouchDB asynchronously
    pub async fn export_ohlc_async(&self, data: &[OHLC]) -> ExportResult<()> {
        let db = self.get_database().await?;
        
        // Convert OHLC data to documents
        let mut documents: Vec<OhlcDocument> = data.iter()
            .map(|ohlc| OhlcDocument::from_ohlc(ohlc, "MARKET"))
            .collect();

        // Bulk insert in batches
        for chunk in documents.chunks_mut(self.batch_size) {
            db.bulk_docs(chunk).await?;
        }

        // Create or update design document with views
        self.create_views(&db).await?;

        Ok(())
    }

    /// Export tick data to CouchDB asynchronously
    pub async fn export_ticks_async(&self, data: &[Tick]) -> ExportResult<()> {
        let db = self.get_database().await?;
        
        // Convert tick data to documents
        let mut documents: Vec<TickDocument> = data.iter()
            .map(|tick| TickDocument::from_tick(tick, "MARKET"))
            .collect();

        // Bulk insert in batches
        for chunk in documents.chunks_mut(self.batch_size) {
            db.bulk_docs(chunk).await?;
        }

        // Create or update design document with views
        self.create_views(&db).await?;

        Ok(())
    }

    /// Create CouchDB views for querying data
    async fn create_views(&self, db: &Database) -> Result<(), CouchError> {
        let design_doc = serde_json::json!({
            "_id": "_design/market_data",
            "views": {
                "by_timestamp": {
                    "map": "function(doc) { if(doc.timestamp) { emit(doc.timestamp, doc); } }"
                },
                "by_symbol_and_timestamp": {
                    "map": "function(doc) { if(doc.symbol && doc.timestamp) { emit([doc.symbol, doc.timestamp], doc); } }"
                },
                "by_type": {
                    "map": "function(doc) { if(doc.doc_type) { emit(doc.doc_type, doc); } }"
                },
                "ohlc_by_date_range": {
                    "map": "function(doc) { if(doc.doc_type === 'ohlc' && doc.timestamp) { emit(doc.timestamp, {open: doc.open, high: doc.high, low: doc.low, close: doc.close, volume: doc.volume}); } }"
                },
                "ticks_by_date_range": {
                    "map": "function(doc) { if(doc.doc_type === 'tick' && doc.timestamp) { emit(doc.timestamp, {price: doc.price, bid: doc.bid, ask: doc.ask, volume: doc.volume}); } }"
                }
            }
        });

        // Try to update or create the design document
        let mut doc = design_doc.clone();
        match db.save(&mut doc).await {
            Ok(_) => Ok(()),
            Err(_) => {
                // If save fails, try to update existing document
                match db.get::<serde_json::Value>("_design/market_data").await {
                    Ok(mut existing) => {
                        existing["views"] = design_doc["views"].clone();
                        db.save(&mut existing).await?;
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }
}

// Synchronous implementation for DataExporter trait
impl DataExporter for CouchDbExporter {
    fn export_ohlc<P: AsRef<Path>>(&self, data: &[OHLC], _path: P) -> ExportResult<()> {
        // Create a runtime for synchronous execution
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.export_ohlc_async(data))
    }

    fn export_ticks<P: AsRef<Path>>(&self, data: &[Tick], _path: P) -> ExportResult<()> {
        // Create a runtime for synchronous execution
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.export_ticks_async(data))
    }
}

/// CouchDB document structure for OHLC data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct OhlcDocument {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    doc_type: String,
    symbol: String,
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl OhlcDocument {
    fn from_ohlc(ohlc: &OHLC, symbol: &str) -> Self {
        let id = format!("ohlc_{}_{}", symbol, ohlc.timestamp);
        Self {
            id,
            rev: None,
            doc_type: "ohlc".to_string(),
            symbol: symbol.to_string(),
            timestamp: ohlc.timestamp,
            open: ohlc.open,
            high: ohlc.high,
            low: ohlc.low,
            close: ohlc.close,
            volume: ohlc.volume.as_f64(),
        }
    }
}

impl TypedCouchDocument for OhlcDocument {
    fn get_id(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn get_rev(&self) -> Cow<'_, str> {
        match &self.rev {
            Some(rev) => Cow::Borrowed(rev),
            None => Cow::Borrowed(""),
        }
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    fn set_rev(&mut self, rev: &str) {
        self.rev = Some(rev.to_string());
    }

    fn merge_ids(&mut self, other: &Self) {
        self.id = other.id.clone();
        self.rev = other.rev.clone();
    }
}

/// CouchDB document structure for tick data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TickDocument {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    doc_type: String,
    symbol: String,
    timestamp: i64,
    price: f64,
    bid: f64,
    ask: f64,
    volume: f64,
}

impl TickDocument {
    fn from_tick(tick: &Tick, symbol: &str) -> Self {
        let id = format!("tick_{}_{}", symbol, tick.timestamp);
        Self {
            id,
            rev: None,
            doc_type: "tick".to_string(),
            symbol: symbol.to_string(),
            timestamp: tick.timestamp,
            price: tick.price,
            bid: tick.bid.unwrap_or(tick.price),
            ask: tick.ask.unwrap_or(tick.price),
            volume: tick.volume.as_f64(),
        }
    }
}

impl TypedCouchDocument for TickDocument {
    fn get_id(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn get_rev(&self) -> Cow<'_, str> {
        match &self.rev {
            Some(rev) => Cow::Borrowed(rev),
            None => Cow::Borrowed(""),
        }
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    fn set_rev(&mut self, rev: &str) {
        self.rev = Some(rev.to_string());
    }

    fn merge_ids(&mut self, other: &Self) {
        self.id = other.id.clone();
        self.rev = other.rev.clone();
    }
}

/// Options for CouchDB export
#[derive(Debug, Clone)]
pub struct CouchDbOptions {
    /// CouchDB server URL
    pub server_url: String,
    /// Database name
    pub database_name: String,
    /// Authentication username
    pub username: Option<String>,
    /// Authentication password
    pub password: Option<String>,
    /// Batch size for bulk operations
    pub batch_size: usize,
}

impl Default for CouchDbOptions {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:5984".to_string(),
            database_name: "market_data".to_string(),
            username: None,
            password: None,
            batch_size: 1000,
        }
    }
}

impl CouchDbOptions {
    /// Create options with custom server URL
    pub fn with_server(mut self, url: impl Into<String>) -> Self {
        self.server_url = url.into();
        self
    }

    /// Set database name
    pub fn with_database(mut self, name: impl Into<String>) -> Self {
        self.database_name = name.into();
        self
    }

    /// Set authentication credentials
    pub fn with_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_couchdb_exporter_creation() {
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_db");
        assert_eq!(exporter.server_url, "http://localhost:5984");
        assert_eq!(exporter.database_name, "test_db");
        assert_eq!(exporter.batch_size, 1000);
    }

    #[test]
    fn test_couchdb_exporter_with_auth() {
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_db")
            .with_auth("admin", "password");
        assert_eq!(exporter.username, Some("admin".to_string()));
        assert_eq!(exporter.password, Some("password".to_string()));
    }

    #[test]
    fn test_couchdb_exporter_with_batch_size() {
        let exporter = CouchDbExporter::new("http://localhost:5984", "test_db")
            .with_batch_size(500);
        assert_eq!(exporter.batch_size, 500);
    }

    #[test]
    fn test_ohlc_document_from_ohlc() {
        use crate::types::Volume;
        let ohlc = OHLC {
            timestamp: 1234567890,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0,
            volume: Volume::new(1000),
        };
        
        let doc = OhlcDocument::from_ohlc(&ohlc, "TEST");
        assert_eq!(doc.id, "ohlc_TEST_1234567890");
        assert_eq!(doc.symbol, "TEST");
        assert_eq!(doc.timestamp, 1234567890);
        assert_eq!(doc.open, 100.0);
        assert_eq!(doc.high, 110.0);
        assert_eq!(doc.low, 95.0);
        assert_eq!(doc.close, 105.0);
        assert_eq!(doc.volume, 1000.0);
    }

    #[test]
    fn test_tick_document_from_tick() {
        use crate::types::Volume;
        let tick = Tick {
            timestamp: 1234567890,
            price: 100.0,
            bid: Some(99.5),
            ask: Some(100.5),
            volume: Volume::new(100),
        };
        
        let doc = TickDocument::from_tick(&tick, "TEST");
        assert_eq!(doc.id, "tick_TEST_1234567890");
        assert_eq!(doc.symbol, "TEST");
        assert_eq!(doc.timestamp, 1234567890);
        assert_eq!(doc.price, 100.0);
        assert_eq!(doc.bid, 99.5);
        assert_eq!(doc.ask, 100.5);
        assert_eq!(doc.volume, 100.0);
    }

    #[test]
    fn test_couchdb_options_default() {
        let options = CouchDbOptions::default();
        assert_eq!(options.server_url, "http://localhost:5984");
        assert_eq!(options.database_name, "market_data");
        assert_eq!(options.batch_size, 1000);
        assert!(options.username.is_none());
        assert!(options.password.is_none());
    }

    #[test]
    fn test_couchdb_options_builder() {
        let options = CouchDbOptions::default()
            .with_server("http://couchdb:5984")
            .with_database("my_data")
            .with_auth("user", "pass")
            .with_batch_size(2000);
        
        assert_eq!(options.server_url, "http://couchdb:5984");
        assert_eq!(options.database_name, "my_data");
        assert_eq!(options.username, Some("user".to_string()));
        assert_eq!(options.password, Some("pass".to_string()));
        assert_eq!(options.batch_size, 2000);
    }
}