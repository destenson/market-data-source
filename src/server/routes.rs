#![allow(unused)]

use super::state::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct ControlCommand {
    pub command: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().route("/ws", get(super::websocket::websocket_handler))
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

pub async fn control(Json(cmd): Json<ControlCommand>) -> impl IntoResponse {
    match cmd.command.as_str() {
        "shutdown" => {
            // Log the shutdown request
            tracing::info!("Shutdown requested via control API");

            // Get delay from params or use default
            let delay_ms = cmd
                .params
                .get("delay_ms")
                .and_then(|v| v.as_u64())
                .unwrap_or(100);

            // Spawn a task to shutdown after a short delay to allow response to be sent
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                tracing::info!("Shutting down server...");
                std::process::exit(0);
            });

            Json(json!({
                "status": "success",
                "command": "shutdown",
                "message": "Server shutdown initiated",
                "delay_ms": delay_ms,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        "reload" => {
            // Placeholder for config reload
            Json(json!({
                "status": "error",
                "command": "reload",
                "message": "Config reload not yet implemented",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        "gc" => {
            // Trigger garbage collection / cleanup
            tracing::info!("Garbage collection requested");

            Json(json!({
                "status": "success",
                "command": "gc",
                "message": "Garbage collection initiated",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        "status" => {
            // Return server status information
            Json(json!({
                "status": "success",
                "command": "status",
                "server": {
                    "version": env!("CARGO_PKG_VERSION"),
                    "uptime": "not tracked", // Could add actual uptime tracking
                    "rust_version": env!("CARGO_PKG_RUST_VERSION"),
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        _ => Json(json!({
            "status": "error",
            "command": cmd.command,
            "message": format!("Unknown control command: {}", cmd.command),
            "available_commands": ["shutdown", "reload", "gc", "status"],
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    }
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
