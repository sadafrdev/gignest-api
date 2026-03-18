use axum::{Json, Router, extract::{Extension, State},routing::post};
use sqlx::PgPool;
use utils::{db::AppState, error::AppError};
use authentication::register::Register;

pub async fn register(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Register>,
) -> Result<(), AppError> {
    form.register(db).await
}

pub fn router() -> Router {
    Router::new().route("/register", post(register))
}
