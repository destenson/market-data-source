use super::models::*;
use crate::config::GeneratorConfig;
use crate::types::{Tick, OHLC};
use utoipa::{openapi, OpenApi};

#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            GenerateRequest,
            GenerateResponse,
            DataFormat,
            MarketDataResponse,
            ResponseMetadata,
            CreateSymbolRequest,
            SymbolInfo,
            SymbolStatistics,
            ApiCapabilities,
            AlgorithmInfo,
            ParameterInfo,
            PresetInfo,
            WebSocketInfo,
            ApiLimits,
            RateLimitInfo,
            HistoricalDataRequest,
            ErrorResponse,
            OHLC,
            Tick,
            GeneratorConfig,
        )
    ),
    tags(
        (name = "Market Data", description = "Market data generation endpoints"),
        (name = "Symbols", description = "Symbol management endpoints"),
        (name = "Export", description = "Data export endpoints"),
        (name = "Discovery", description = "API discovery and capabilities"),
    ),
    info(
        title = "Market Data Source API",
        version = env!("CARGO_PKG_VERSION"),
        description = "Synthetic market data generation server with REST and WebSocket APIs",
        contact(
            name = "Market Data Source",
            url = "https://github.com/yourusername/market-data-source",
        ),
        license(
            name = "MIT",
        ),
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.marketdata.example.com", description = "Production server"),
    ),
)]
pub struct ApiDoc;

pub fn openapi_spec() -> openapi::OpenApi {
    ApiDoc::openapi()
}
