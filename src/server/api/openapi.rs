use utoipa::{OpenApi, openapi};
use super::models::*;
use crate::types::{OHLC, Tick};
use crate::config::GeneratorConfig;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::handlers::list_symbols,
        super::handlers::create_symbol,
        super::handlers::delete_symbol,
        super::handlers::generate_data,
        super::handlers::stream_data,
        super::handlers::get_historical,
        super::handlers::configure_generator,
        super::handlers::export_csv,
        super::handlers::export_json,
        super::handlers::export_png,
        super::handlers::get_capabilities,
        super::handlers::list_algorithms,
        super::handlers::list_presets,
    ),
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