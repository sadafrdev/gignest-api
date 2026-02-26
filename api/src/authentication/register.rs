use authentication::register::Register;
use axum::Json;
use axum::Router;
use axum::routing::post;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(form): Json<Register>,
) -> Result<(), StatusCode> {
    form.register(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .layer(Extension(state))
}
