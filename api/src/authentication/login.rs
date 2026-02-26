use authentication::login::Login;
use axum::Json;
use axum::Router;
use axum::extract::Extension;
use axum::routing::get;
use utils::{db::AppState, error::AppError};

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(form): Json<Login>,
) -> Result<(), AppError> {
    form.login(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login))
        .layer(Extension(state))
}
