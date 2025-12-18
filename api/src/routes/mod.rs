use crate::AppState;
use crate::authentication;
use axum::{Router, routing::post};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(authentication::login))
        .with_state(state)
}
