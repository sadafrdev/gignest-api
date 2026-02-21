use axum::Json;
use axum::routing::post;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;
use authentication::register::Register;
use axum::Router;

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Register>,
) -> Result<(), StatusCode> {
    Register::register(Extension(state), Json(payload)).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .layer(Extension(state))
}