use axum::{
    extract::{ws::{WebSocketUpgrade, WebSocket, Message}, State},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
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
        let _ = sender.send(Message::Text(msg.into())).await;
    }
    
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let mut subscriptions: Vec<(String, tokio::task::JoinHandle<()>)> = Vec::new();
    
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<WsMessage>(&text) {
                            Ok(ws_msg) => {
                                match ws_msg {
                                    WsMessage::Subscribe { symbol, interval } => {
                                        info!("WebSocket subscribing to {} with interval {}ms", symbol, interval);
                                        
                                        // Cancel existing subscription for this symbol
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
                                        let tx_clone = tx.clone();
                                        
                                        let handle = tokio::spawn(async move {
                                            let mut interval_timer = tokio::time::interval(
                                                std::time::Duration::from_millis(interval)
                                            );
                                            
                                            loop {
                                                interval_timer.tick().await;
                                                
                                                let ohlc = {
                                                    let mut generator_lock = generator.write().await;
                                                    generator_lock.generate_ohlc()
                                                };
                                                
                                                let response = WsResponse::MarketData {
                                                    symbol: symbol_clone.clone(),
                                                    ohlc,
                                                    timestamp: chrono::Utc::now(),
                                                };
                                                
                                                if let Ok(msg) = serde_json::to_string(&response) {
                                                    if tx_clone.send(msg).is_err() {
                                                        break;
                                                    }
                                                }
                                            }
                                        });
                                        
                                        subscriptions.push((symbol.clone(), handle));
                                        
                                        let response = WsResponse::Subscribed { symbol, interval };
                                        if let Ok(msg) = serde_json::to_string(&response) {
                                            let _ = sender.send(Message::Text(msg.into())).await;
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
                                            let _ = sender.send(Message::Text(msg.into())).await;
                                        }
                                    }
                                    WsMessage::Configure { symbol, config } => {
                                        debug!("Configuring {} with {:?}", symbol, config);
                                        
                                        let response = WsResponse::Error {
                                            message: "Configuration not yet implemented".to_string(),
                                        };
                                        if let Ok(msg) = serde_json::to_string(&response) {
                                            let _ = sender.send(Message::Text(msg.into())).await;
                                        }
                                    }
                                    WsMessage::Ping => {
                                        let response = WsResponse::Pong;
                                        if let Ok(msg) = serde_json::to_string(&response) {
                                            let _ = sender.send(Message::Text(msg.into())).await;
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
                                    let _ = sender.send(Message::Text(msg.into())).await;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            
            // Handle outgoing messages from subscriptions
            Some(msg) = rx.recv() => {
                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        }
    }
    
    // Clean up subscriptions
    for (_, handle) in subscriptions {
        handle.abort();
    }
    
    info!("WebSocket connection closed");
}