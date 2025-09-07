#![allow(unused)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response, Sse},
    Json,
};
use futures::stream::{self, Stream};
use serde_json::json;
use std::convert::Infallible;
use std::time::Duration;
use super::models::*;
use crate::server::state::AppState;
use crate::GeneratorConfig;

pub async fn list_symbols(State(state): State<AppState>) -> impl IntoResponse {
    let symbols = state.list_symbols().await;
    Json(json!({
        "symbols": symbols,
        "count": symbols.len()
    }))
}

pub async fn create_symbol(
    State(state): State<AppState>,
    Json(req): Json<CreateSymbolRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let mut config = req.config.unwrap_or_default();
    
    // Apply smart defaults based on what was provided
    config.apply_smart_defaults();
    
    // Validate the configuration
    if let Err(e) = config.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid configuration: {e}"),
                code: 400,
                details: Some(serde_json::json!({
                    "validation_error": e.to_string(),
                    "config": config
                })),
            })
        ));
    }
    
    state.create_generator_with_config(&req.symbol, config.clone()).await;
    
    let info = SymbolInfo {
        symbol: req.symbol.clone(),
        active: true,
        created_at: chrono::Utc::now(),
        config,
        statistics: None,
    };
    
    Ok((StatusCode::CREATED, Json(info)))
}

pub async fn delete_symbol(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    match state.remove_generator(&symbol).await {
        Some(_) => StatusCode::NO_CONTENT.into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Symbol {symbol} not found"),
                code: 404,
                details: None,
            })
        ).into_response()
    }
}

pub async fn generate_data(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
    Json(req): Json<GenerateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let generator = state.get_or_create_generator(&symbol).await;
    let mut generator_lock = generator.write().await;
    
    let data = match req.format {
        DataFormat::Ohlc => {
            let ohlc = generator_lock.generate_series(req.count);
            MarketDataResponse::Ohlc(ohlc)
        }
        DataFormat::Tick => {
            let ticks = generator_lock.generate_ticks(req.count);
            MarketDataResponse::Tick(ticks)
        }
        DataFormat::Both => {
            let ohlc = generator_lock.generate_series(req.count);
            let ticks = generator_lock.generate_ticks(req.count * 10);
            MarketDataResponse::Both { ohlc, ticks }
        }
    };
    
    let response = GenerateResponse {
        symbol: symbol.clone(),
        data,
        metadata: ResponseMetadata {
            generated_at: chrono::Utc::now(),
            count: req.count,
            format: format!("{:?}", req.format).to_lowercase(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
        },
    };
    
    Ok(Json(response))
}

pub async fn stream_data(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Sse<impl Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    let generator = state.get_or_create_generator(&symbol).await;
    
    let stream = stream::unfold(
        (generator, symbol),
        |(generator, symbol)| async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            
            let ohlc = {
                let mut generator_lock = generator.write().await;
                generator_lock.generate_ohlc()
            };
            
            let data = json!({
                "symbol": symbol,
                "ohlc": ohlc,
                "timestamp": chrono::Utc::now()
            });
            
            let event = axum::response::sse::Event::default()
                .data(data.to_string());
            
            Some((Ok(event), (generator, symbol)))
        }
    );
    
    Sse::new(stream)
}

pub async fn get_historical(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
    Query(params): Query<HistoricalDataRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let generator = state.get_or_create_generator(&symbol).await;
    let mut generator_lock = generator.write().await;
    
    let count = params.limit.unwrap_or(100);
    let ohlc = generator_lock.generate_series(count);
    
    Ok(Json(json!({
        "symbol": symbol,
        "data": ohlc,
        "count": ohlc.len(),
        "interval": params.interval.unwrap_or_else(|| "1m".to_string()),
        "from": params.from,
        "to": params.to
    })))
}

pub async fn configure_generator(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
    Json(config): Json<GeneratorConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    state.create_generator_with_config(&symbol, config.clone()).await;
    
    Ok(Json(json!({
        "symbol": symbol,
        "config": config,
        "status": "configured"
    })))
}

pub async fn export_csv(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    #[cfg(feature = "csv_export")]
    {
        use crate::export::to_csv_string_ohlc;
        
        let generator = state.get_or_create_generator(&symbol).await;
        let mut generator_lock = generator.write().await;
        let ohlc = generator_lock.generate_series(100);
        
        match to_csv_string_ohlc(&ohlc) {
            Ok(csv) => Ok((
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "text/csv")],
                csv
            ).into_response()),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to generate CSV: {e}"),
                    code: 500,
                    details: None,
                })
            ))
        }
    }
    
    #[cfg(not(feature = "csv_export"))]
    {
        Err((
            StatusCode::NOT_IMPLEMENTED,
            Json(ErrorResponse {
                error: "CSV export feature not enabled".to_string(),
                code: 501,
                details: None,
            })
        ))
    }
}

pub async fn export_json(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let generator = state.get_or_create_generator(&symbol).await;
    let mut generator_lock = generator.write().await;
    let ohlc = generator_lock.generate_series(100);
    
    Ok(Json(json!({
        "symbol": symbol,
        "data": ohlc,
        "generated_at": chrono::Utc::now()
    })))
}

pub async fn export_png(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    #[cfg(feature = "png_export")]
    {
        use crate::export::chart::ChartBuilder;
        
        let generator = state.get_or_create_generator(&symbol).await;
        let mut generator_lock = generator.write().await;
        let ohlc = generator_lock.generate_series(100);
        
        let mut buffer = Vec::new();
        let chart = ChartBuilder::new()
            .title(format!("{symbol} Market Data"))
            .build_to_buffer(&ohlc, &mut buffer);
        
        match chart {
            Ok(_) => Ok((
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "image/png")],
                buffer
            ).into_response()),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to generate chart: {e}"),
                    code: 500,
                    details: None,
                })
            ))
        }
    }
    
    #[cfg(not(feature = "png_export"))]
    {
        Err((
            StatusCode::NOT_IMPLEMENTED,
            Json(ErrorResponse {
                error: "PNG export feature not enabled".to_string(),
                code: 501,
                details: None,
            })
        ))
    }
}

pub async fn get_capabilities(State(state): State<AppState>) -> impl IntoResponse {
    let capabilities = ApiCapabilities {
        version: env!("CARGO_PKG_VERSION").to_string(),
        features: vec![
            "market_data_generation".to_string(),
            "websocket_streaming".to_string(),
            "multiple_symbols".to_string(),
            "configurable_generators".to_string(),
            "export_formats".to_string(),
            "runtime_discovery".to_string(),
        ],
        export_formats: vec![
            "json".to_string(),
            #[cfg(feature = "csv_export")]
            "csv".to_string(),
            #[cfg(feature = "png_export")]
            "png".to_string(),
        ],
        algorithms: vec![
            AlgorithmInfo {
                name: "random_walk".to_string(),
                description: "Random walk with drift algorithm".to_string(),
                parameters: vec![
                    ParameterInfo {
                        name: "initial_price".to_string(),
                        data_type: "decimal".to_string(),
                        description: "Starting price for the generator".to_string(),
                        default_value: json!(100.0),
                        constraints: Some(json!({"min": 0.01, "max": 1000000.0})),
                    },
                    ParameterInfo {
                        name: "volatility".to_string(),
                        data_type: "decimal".to_string(),
                        description: "Price volatility factor".to_string(),
                        default_value: json!(0.02),
                        constraints: Some(json!({"min": 0.0, "max": 1.0})),
                    },
                    ParameterInfo {
                        name: "drift".to_string(),
                        data_type: "decimal".to_string(),
                        description: "Directional bias in price movement".to_string(),
                        default_value: json!(0.0001),
                        constraints: Some(json!({"min": -0.1, "max": 0.1})),
                    },
                ],
            },
        ],
        presets: vec![
            PresetInfo {
                name: "volatile_crypto".to_string(),
                description: "High volatility suitable for cryptocurrency".to_string(),
                algorithm: "random_walk".to_string(),
                suitable_for: vec!["BTC".to_string(), "ETH".to_string(), "crypto".to_string()],
            },
            PresetInfo {
                name: "stable_forex".to_string(),
                description: "Low volatility suitable for forex pairs".to_string(),
                algorithm: "random_walk".to_string(),
                suitable_for: vec!["EUR/USD".to_string(), "forex".to_string()],
            },
            PresetInfo {
                name: "stock_market".to_string(),
                description: "Moderate volatility for stock market simulation".to_string(),
                algorithm: "random_walk".to_string(),
                suitable_for: vec!["AAPL".to_string(), "GOOGL".to_string(), "stocks".to_string()],
            },
        ],
        websocket: WebSocketInfo {
            endpoint: "/ws".to_string(),
            protocols: vec!["market-data-v1".to_string()],
            message_types: vec![
                "subscribe".to_string(),
                "unsubscribe".to_string(),
                "configure".to_string(),
                "ping".to_string(),
            ],
            max_subscriptions_per_connection: 10,
        },
        limits: ApiLimits {
            max_data_points_per_request: 10000,
            max_symbols: 100,
            max_websocket_connections: 1000,
            rate_limit: state.config.rate_limit.as_ref().map(|rl| RateLimitInfo {
                requests_per_second: rl.requests_per_second,
                burst_size: rl.burst_size,
            }),
        },
    };
    
    Json(capabilities)
}

pub async fn list_algorithms() -> impl IntoResponse {
    Json(json!({
        "algorithms": [
            {
                "id": "random_walk",
                "name": "Random Walk with Drift",
                "description": "Generates price movements using geometric Brownian motion",
                "parameters": {
                    "volatility": {
                        "type": "float",
                        "range": [0.0, 1.0],
                        "default": 0.02,
                        "description": "Controls price movement magnitude"
                    },
                    "drift": {
                        "type": "float",
                        "range": [-0.1, 0.1],
                        "default": 0.0001,
                        "description": "Directional bias in price movement"
                    }
                },
                "suitable_for": ["stocks", "forex", "crypto", "commodities"]
            }
        ],
        "planned": [
            "garch",
            "mean_reversion",
            "jump_diffusion",
            "regime_switching"
        ]
    }))
}

pub async fn list_presets() -> impl IntoResponse {
    Json(json!({
        "presets": {
            "volatile_crypto": {
                "description": "High volatility cryptocurrency market",
                "config": {
                    "initial_price": 50000.0,
                    "volatility": 0.05,
                    "drift": 0.0002,
                    "volume_volatility": 0.15
                }
            },
            "stable_forex": {
                "description": "Low volatility forex pair",
                "config": {
                    "initial_price": 1.2,
                    "volatility": 0.005,
                    "drift": 0.00001,
                    "volume_volatility": 0.05
                }
            },
            "stock_market": {
                "description": "Typical stock market behavior",
                "config": {
                    "initial_price": 150.0,
                    "volatility": 0.02,
                    "drift": 0.0001,
                    "volume_volatility": 0.1
                }
            },
            "commodity": {
                "description": "Commodity futures market",
                "config": {
                    "initial_price": 75.0,
                    "volatility": 0.03,
                    "drift": 0.00005,
                    "volume_volatility": 0.12
                }
            }
        }
    }))
}
