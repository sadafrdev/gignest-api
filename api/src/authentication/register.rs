use authentication::register::Register;
use axum::Json;
use axum::Router;
use axum::routing::post;
use axum::extract::Extension;
use utils::{db::AppState, error::AppError};

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(form): Json<Register>,
) -> Result<(), AppError> {
    form.register(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .layer(Extension(state))
}
