use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::{GeneratorConfig, OHLC, Tick};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerateRequest {
    pub count: usize,
    #[serde(default)]
    pub format: DataFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GeneratorConfig>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum DataFormat {
    #[default]
    Ohlc,
    Tick,
    Both,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerateResponse {
    pub symbol: String,
    pub data: MarketDataResponse,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum MarketDataResponse {
    Ohlc(Vec<OHLC>),
    Tick(Vec<Tick>),
    Both {
        ohlc: Vec<OHLC>,
        ticks: Vec<Tick>,
    },
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseMetadata {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub count: usize,
    pub format: String,
    pub generator_version: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSymbolRequest {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GeneratorConfig>,
    #[serde(default)]
    pub auto_start: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SymbolInfo {
    pub symbol: String,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub config: GeneratorConfig,
    pub statistics: Option<SymbolStatistics>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SymbolStatistics {
    pub total_generated: u64,
    pub last_price: Decimal,
    pub high_24h: Decimal,
    pub low_24h: Decimal,
    pub volume_24h: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiCapabilities {
    pub version: String,
    pub features: Vec<String>,
    pub export_formats: Vec<String>,
    pub algorithms: Vec<AlgorithmInfo>,
    pub presets: Vec<PresetInfo>,
    pub websocket: WebSocketInfo,
    pub limits: ApiLimits,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AlgorithmInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParameterInfo {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub default_value: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PresetInfo {
    pub name: String,
    pub description: String,
    pub algorithm: String,
    pub suitable_for: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WebSocketInfo {
    pub endpoint: String,
    pub protocols: Vec<String>,
    pub message_types: Vec<String>,
    pub max_subscriptions_per_connection: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiLimits {
    pub max_data_points_per_request: u32,
    pub max_symbols: u32,
    pub max_websocket_connections: u32,
    pub rate_limit: Option<RateLimitInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RateLimitInfo {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HistoricalDataRequest {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
    pub interval: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    pub details: Option<serde_json::Value>,
}