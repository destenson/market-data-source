pub mod handlers;
pub mod models;
pub mod openapi;

use axum::{
    Router,
    routing::{get, post, delete},
};
use super::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/symbols", get(handlers::list_symbols).post(handlers::create_symbol))
        .route("/symbols/{symbol}", delete(handlers::delete_symbol))
        .route("/generate/{symbol}", post(handlers::generate_data))
        .route("/stream/{symbol}", get(handlers::stream_data))
        .route("/historical/{symbol}", get(handlers::get_historical))
        .route("/configure/{symbol}", post(handlers::configure_generator))
        .route("/export/{symbol}/csv", get(handlers::export_csv))
        .route("/export/{symbol}/json", get(handlers::export_json))
        .route("/export/{symbol}/png", get(handlers::export_png))
        .route("/capabilities", get(handlers::get_capabilities))
        .route("/algorithms", get(handlers::list_algorithms))
        .route("/presets", get(handlers::list_presets))
}
