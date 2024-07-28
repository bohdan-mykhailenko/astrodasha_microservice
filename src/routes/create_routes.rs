use axum::{routing::get, Router};
use crate::api;


pub fn create_routers() -> Router {
    Router::new()
        .route("/api/v1/astro_position", get(api::handlers::get_positions))
}