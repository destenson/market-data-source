use crate::{MarketDataGenerator, GeneratorConfig};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use super::config::ServerConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub generators: Arc<RwLock<HashMap<String, Arc<RwLock<MarketDataGenerator>>>>>,
    pub subscriptions: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            generators: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn get_or_create_generator(&self, symbol: &str) -> Arc<RwLock<MarketDataGenerator>> {
        let mut generators = self.generators.write().await;
        
        generators.entry(symbol.to_string())
            .or_insert_with(|| {
                let config = GeneratorConfig::default();
                Arc::new(RwLock::new(MarketDataGenerator::with_config(config)))
            })
            .clone()
    }
    
    pub async fn create_generator_with_config(&self, symbol: &str, config: GeneratorConfig) -> Arc<RwLock<MarketDataGenerator>> {
        let mut generators = self.generators.write().await;
        let generator = Arc::new(RwLock::new(MarketDataGenerator::with_config(config)));
        generators.insert(symbol.to_string(), generator.clone());
        generator
    }
    
    pub async fn remove_generator(&self, symbol: &str) -> Option<Arc<RwLock<MarketDataGenerator>>> {
        let mut generators = self.generators.write().await;
        generators.remove(symbol)
    }
    
    pub async fn list_symbols(&self) -> Vec<String> {
        let generators = self.generators.read().await;
        generators.keys().cloned().collect()
    }
}