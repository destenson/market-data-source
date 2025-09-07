//! Environment variable configuration for Market Data Source
//!
//! This module provides centralized management of environment variables
//! for API keys, database credentials, and other configuration settings.

use std::env;
use std::path::PathBuf;

#[cfg(feature = "dotenvy")]
use dotenvy;

/// Environment configuration for the application
#[derive(Debug, Clone)]
pub struct EnvConfig {
    // CouchDB Configuration
    pub couchdb_url: Option<String>,
    pub couchdb_username: Option<String>,
    pub couchdb_password: Option<String>,
    pub couchdb_database: Option<String>,
    
    // API Keys for future data sources
    pub alpha_vantage_api_key: Option<String>,
    pub polygon_api_key: Option<String>,
    pub finnhub_api_key: Option<String>,
    pub iex_cloud_api_key: Option<String>,
    pub twelve_data_api_key: Option<String>,
    pub yahoo_finance_api_key: Option<String>,
    pub quandl_api_key: Option<String>,
    
    // Binance API (for crypto data)
    pub binance_api_key: Option<String>,
    pub binance_secret_key: Option<String>,
    
    // Coinbase API
    pub coinbase_api_key: Option<String>,
    pub coinbase_secret_key: Option<String>,
    
    // General Configuration
    pub data_directory: Option<PathBuf>,
    pub log_level: Option<String>,
    pub max_retries: Option<u32>,
    pub request_timeout: Option<u64>,
    
    // Export Configuration
    pub default_export_format: Option<String>,
    pub export_batch_size: Option<usize>,
}

impl EnvConfig {
    /// Load configuration from environment variables
    pub fn load() -> Self {
        // Try to load .env file if the feature is enabled
        #[cfg(feature = "dotenvy")]
        let _ = dotenvy::dotenv();
        
        Self {
            // CouchDB Configuration
            couchdb_url: env::var("COUCHDB_URL").ok()
                .or_else(|| env::var("COUCH_URL").ok()),
            couchdb_username: env::var("COUCHDB_USERNAME").ok()
                .or_else(|| env::var("COUCHDB_USER").ok()),
            couchdb_password: env::var("COUCHDB_PASSWORD").ok()
                .or_else(|| env::var("COUCHDB_PASS").ok()),
            couchdb_database: env::var("COUCHDB_DATABASE").ok()
                .or_else(|| env::var("COUCHDB_DB").ok()),
            
            // API Keys
            alpha_vantage_api_key: env::var("ALPHA_VANTAGE_API_KEY").ok(),
            polygon_api_key: env::var("POLYGON_API_KEY").ok(),
            finnhub_api_key: env::var("FINNHUB_API_KEY").ok(),
            iex_cloud_api_key: env::var("IEX_CLOUD_API_KEY").ok()
                .or_else(|| env::var("IEX_API_KEY").ok()),
            twelve_data_api_key: env::var("TWELVE_DATA_API_KEY").ok(),
            yahoo_finance_api_key: env::var("YAHOO_FINANCE_API_KEY").ok(),
            quandl_api_key: env::var("QUANDL_API_KEY").ok(),
            
            // Binance API
            binance_api_key: env::var("BINANCE_API_KEY").ok(),
            binance_secret_key: env::var("BINANCE_SECRET_KEY").ok()
                .or_else(|| env::var("BINANCE_SECRET").ok()),
            
            // Coinbase API
            coinbase_api_key: env::var("COINBASE_API_KEY").ok(),
            coinbase_secret_key: env::var("COINBASE_SECRET_KEY").ok()
                .or_else(|| env::var("COINBASE_SECRET").ok()),
            
            // General Configuration
            data_directory: env::var("DATA_DIRECTORY").ok()
                .or_else(|| env::var("DATA_DIR").ok())
                .map(PathBuf::from),
            log_level: env::var("LOG_LEVEL").ok()
                .or_else(|| env::var("RUST_LOG").ok()),
            max_retries: env::var("MAX_RETRIES").ok()
                .and_then(|s| s.parse().ok()),
            request_timeout: env::var("REQUEST_TIMEOUT").ok()
                .and_then(|s| s.parse().ok()),
            
            // Export Configuration
            default_export_format: env::var("DEFAULT_EXPORT_FORMAT").ok(),
            export_batch_size: env::var("EXPORT_BATCH_SIZE").ok()
                .and_then(|s| s.parse().ok()),
        }
    }
    
    /// Load configuration from a specific .env file
    #[cfg(feature = "dotenvy")]
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, dotenvy::Error> {
        dotenvy::from_path(path)?;
        Ok(Self::load())
    }
    
    /// Get CouchDB URL with fallback to default
    pub fn couchdb_url(&self) -> String {
        self.couchdb_url.clone()
            .unwrap_or_else(|| "http://localhost:5984".to_string())
    }
    
    /// Get CouchDB database name with fallback to default
    pub fn couchdb_database(&self) -> String {
        self.couchdb_database.clone()
            .unwrap_or_else(|| "market_data".to_string())
    }
    
    /// Check if CouchDB credentials are available
    pub fn has_couchdb_auth(&self) -> bool {
        self.couchdb_username.is_some() && self.couchdb_password.is_some()
    }
    
    /// Get CouchDB credentials as a tuple
    pub fn couchdb_credentials(&self) -> Option<(String, String)> {
        match (&self.couchdb_username, &self.couchdb_password) {
            (Some(user), Some(pass)) => Some((user.clone(), pass.clone())),
            _ => None,
        }
    }
    
    /// Get export batch size with fallback to default
    pub fn export_batch_size(&self) -> usize {
        self.export_batch_size.unwrap_or(1000)
    }
    
    /// Get request timeout in seconds with fallback to default
    pub fn request_timeout_secs(&self) -> u64 {
        self.request_timeout.unwrap_or(30)
    }
    
    /// Get max retries with fallback to default
    pub fn max_retries(&self) -> u32 {
        self.max_retries.unwrap_or(3)
    }
    
    /// Validate that required environment variables are set
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let missing = Vec::new();
        
        // Add validation for required variables here
        // For now, most variables are optional
        
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    
    /// Check if a specific API key is configured
    pub fn has_api_key(&self, provider: &str) -> bool {
        match provider.to_lowercase().as_str() {
            "alpha_vantage" | "alphavantage" => self.alpha_vantage_api_key.is_some(),
            "polygon" => self.polygon_api_key.is_some(),
            "finnhub" => self.finnhub_api_key.is_some(),
            "iex" | "iex_cloud" => self.iex_cloud_api_key.is_some(),
            "twelve_data" | "twelvedata" => self.twelve_data_api_key.is_some(),
            "yahoo" | "yahoo_finance" => self.yahoo_finance_api_key.is_some(),
            "quandl" => self.quandl_api_key.is_some(),
            "binance" => self.binance_api_key.is_some(),
            "coinbase" => self.coinbase_api_key.is_some(),
            _ => false,
        }
    }
    
    /// Get API key for a specific provider
    pub fn get_api_key(&self, provider: &str) -> Option<String> {
        match provider.to_lowercase().as_str() {
            "alpha_vantage" | "alphavantage" => self.alpha_vantage_api_key.clone(),
            "polygon" => self.polygon_api_key.clone(),
            "finnhub" => self.finnhub_api_key.clone(),
            "iex" | "iex_cloud" => self.iex_cloud_api_key.clone(),
            "twelve_data" | "twelvedata" => self.twelve_data_api_key.clone(),
            "yahoo" | "yahoo_finance" => self.yahoo_finance_api_key.clone(),
            "quandl" => self.quandl_api_key.clone(),
            "binance" => self.binance_api_key.clone(),
            "coinbase" => self.coinbase_api_key.clone(),
            _ => None,
        }
    }
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self::load()
    }
}

// Removed global singleton - just use EnvConfig::load() when needed

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_env_config_load() {
        let _config = EnvConfig::load();
        // Basic load test - should not panic
        assert!(true);
    }
    
    #[test]
    fn test_default_values() {
        // Just test that the defaults work
        let config = EnvConfig::default();
        // These are hardcoded defaults, not from environment
        assert_eq!(config.request_timeout_secs(), 30);
        assert_eq!(config.max_retries(), 3);
    }
    
    #[test]
    fn test_has_api_key() {
        let config = EnvConfig::load();
        // These should return false unless env vars are set
        assert!(!config.has_api_key("unknown"));
    }
    
    #[test]
    fn test_validation() {
        let config = EnvConfig::load();
        // Currently no required variables, so should pass
        assert!(config.validate().is_ok());
    }
}