use axum::{
    extract::{ws::WebSocketUpgrade, State, WebSocket},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use super::state::AppState;
use crate::types::OHLC;
use tracing::{debug, error, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    Subscribe { 
        symbol: String,
        #[serde(default = "default_interval")]
        interval: u64,
    },
    Unsubscribe { symbol: String },
    Configure { symbol: String, config: serde_json::Value },
    Ping,
    Pong,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum WsResponse {
    MarketData {
        symbol: String,
        ohlc: OHLC,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    Subscribed {
        symbol: String,
        interval: u64,
    },
    Unsubscribed {
        symbol: String,
    },
    Error {
        message: String,
    },
    Pong,
    Welcome {
        version: String,
        capabilities: Vec<String>,
    },
}

fn default_interval() -> u64 {
    1000
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    let welcome = WsResponse::Welcome {
        version: "1.0.0".to_string(),
        capabilities: vec![
            "subscribe".to_string(),
            "unsubscribe".to_string(),
            "configure".to_string(),
            "streaming".to_string(),
        ],
    };
    
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
    }
    
    let mut subscriptions: Vec<(String, tokio::task::JoinHandle<()>)> = Vec::new();
    
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_msg) => {
                            match ws_msg {
                                WsMessage::Subscribe { symbol, interval } => {
                                    info!("WebSocket subscribing to {} with interval {}ms", symbol, interval);
                                    
                                    subscriptions.retain(|(s, handle)| {
                                        if s == &symbol {
                                            handle.abort();
                                            false
                                        } else {
                                            true
                                        }
                                    });
                                    
                                    let generator = state.get_or_create_generator(&symbol).await;
                                    let symbol_clone = symbol.clone();
                                    let mut sender_clone = sender.clone();
                                    
                                    let handle = tokio::spawn(async move {
                                        let mut interval_timer = tokio::time::interval(
                                            std::time::Duration::from_millis(interval)
                                        );
                                        
                                        loop {
                                            interval_timer.tick().await;
                                            
                                            let mut gen = generator.write().await;
                                            let ohlc = gen.generate_ohlc();
                                            
                                            let response = WsResponse::MarketData {
                                                symbol: symbol_clone.clone(),
                                                ohlc,
                                                timestamp: chrono::Utc::now(),
                                            };
                                            
                                            if let Ok(msg) = serde_json::to_string(&response) {
                                                if sender_clone.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                                                    break;
                                                }
                                            }
                                        }
                                    });
                                    
                                    subscriptions.push((symbol.clone(), handle));
                                    
                                    let response = WsResponse::Subscribed { symbol, interval };
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
                                    }
                                }
                                WsMessage::Unsubscribe { symbol } => {
                                    info!("WebSocket unsubscribing from {}", symbol);
                                    
                                    subscriptions.retain(|(s, handle)| {
                                        if s == &symbol {
                                            handle.abort();
                                            false
                                        } else {
                                            true
                                        }
                                    });
                                    
                                    let response = WsResponse::Unsubscribed { symbol };
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
                                    }
                                }
                                WsMessage::Configure { symbol, config } => {
                                    debug!("Configuring {} with {:?}", symbol, config);
                                    
                                    let response = WsResponse::Error {
                                        message: "Configuration not yet implemented".to_string(),
                                    };
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
                                    }
                                }
                                WsMessage::Ping => {
                                    let response = WsResponse::Pong;
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
                                    }
                                }
                                WsMessage::Pong => {}
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse WebSocket message: {}", e);
                            let response = WsResponse::Error {
                                message: format!("Invalid message format: {}", e),
                            };
                            if let Ok(msg) = serde_json::to_string(&response) {
                                let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
                            }
                        }
                    }
                }
                axum::extract::ws::Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    }
    
    for (_, handle) in subscriptions {
        handle.abort();
    }
    
    info!("WebSocket connection closed");
}