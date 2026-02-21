use authentication::login::Login;
use axum::Json;
use axum::Router;
use axum::routing::get;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Login>,
) -> Result<(), StatusCode> {
    Login::login(Extension(state), Json(payload)).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login))
        .layer(Extension(state))
}
