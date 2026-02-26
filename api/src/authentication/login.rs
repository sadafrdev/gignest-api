use authentication::login::Login;
use axum::Json;
use axum::Router;
use axum::routing::get;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(form): Json<Login>,
) -> Result<(), StatusCode> {
    form.login(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login))
        .layer(Extension(state))
}
