#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub enable_websocket: bool,
    pub enable_swagger: bool,
    pub max_connections: usize,
    pub rate_limit: Option<RateLimitConfig>,
    pub cors_origins: Vec<String>,
    pub api_prefix: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            enable_websocket: true,
            enable_swagger: true,
            max_connections: 1000,
            rate_limit: Some(RateLimitConfig {
                requests_per_second: 100,
                burst_size: 200,
            }),
            cors_origins: vec!["*".to_string()],
            api_prefix: "/api/v1".to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(port) = std::env::var("SERVER_PORT") {
            if let Ok(p) = port.parse() {
                config.port = p;
            }
        }
        
        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.host = host;
        }
        
        if let Ok(ws) = std::env::var("ENABLE_WEBSOCKET") {
            config.enable_websocket = ws.parse().unwrap_or(true);
        }
        
        if let Ok(swagger) = std::env::var("ENABLE_SWAGGER") {
            config.enable_swagger = swagger.parse().unwrap_or(true);
        }
        
        if let Ok(level) = std::env::var("LOG_LEVEL") {
            config.log_level = level;
        }
        
        config
    }
    
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
