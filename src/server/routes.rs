use axum::{
    extract::State,
    response::{Json, IntoResponse},
    routing::get,
    Router,
};
use serde_json::json;
use super::state::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
}

pub fn ws_routes() -> Router<AppState> {
    Router::new()
        .route("/ws", get(super::websocket::websocket_handler))
}

pub async fn index() -> impl IntoResponse {
    Json(json!({
        "name": "Market Data Source Server",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Synthetic market data generation server with REST and WebSocket APIs",
        "api_docs": "/swagger-ui",
        "api_discovery": "/api",
        "health": "/health",
        "websocket": "/ws"
    }))
}

pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn api_discovery(State(state): State<AppState>) -> impl IntoResponse {
    let symbols = state.list_symbols().await;
    
    Json(json!({
        "version": "v1",
        "base_url": format!("{}", state.config.api_prefix),
        "endpoints": {
            "market_data": {
                "generate": {
                    "method": "POST",
                    "path": "/generate/{symbol}",
                    "description": "Generate market data for a symbol"
                },
                "stream": {
                    "method": "GET",
                    "path": "/stream/{symbol}",
                    "description": "Stream real-time market data"
                },
                "historical": {
                    "method": "GET",
                    "path": "/historical/{symbol}",
                    "description": "Get historical market data"
                },
                "configure": {
                    "method": "POST",
                    "path": "/configure/{symbol}",
                    "description": "Configure generator for a symbol"
                }
            },
            "symbols": {
                "list": {
                    "method": "GET",
                    "path": "/symbols",
                    "description": "List all available symbols"
                },
                "create": {
                    "method": "POST",
                    "path": "/symbols",
                    "description": "Create a new symbol generator"
                },
                "delete": {
                    "method": "DELETE",
                    "path": "/symbols/{symbol}",
                    "description": "Delete a symbol generator"
                }
            },
            "export": {
                "csv": {
                    "method": "GET",
                    "path": "/export/{symbol}/csv",
                    "description": "Export data as CSV"
                },
                "json": {
                    "method": "GET",
                    "path": "/export/{symbol}/json",
                    "description": "Export data as JSON"
                }
            }
        },
        "websocket": {
            "url": "/ws",
            "protocols": ["market-data-v1"],
            "messages": {
                "subscribe": {
                    "type": "subscribe",
                    "payload": {"symbol": "string", "interval": "string"}
                },
                "unsubscribe": {
                    "type": "unsubscribe",
                    "payload": {"symbol": "string"}
                }
            }
        },
        "active_symbols": symbols,
        "features": {
            "websocket": state.config.enable_websocket,
            "swagger": state.config.enable_swagger,
            "rate_limiting": state.config.rate_limit.is_some()
        },
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "openapi_spec": "/api-docs/openapi.json"
        }
    }))
}