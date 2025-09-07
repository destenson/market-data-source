pub mod api;
pub mod config;
pub mod routes;
pub mod state;
pub mod websocket;

pub use config::ServerConfig;
pub use state::AppState;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

pub async fn run_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new(config.clone());

    let app = Router::new()
        .merge(routes::api_routes())
        .merge(routes::ws_routes())
        .nest("/api/v1", api::routes())
        .route("/", get(routes::index))
        .route("/health", get(routes::health))
        .route("/control", post(routes::control))
        .route("/api", get(routes::api_discovery))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
