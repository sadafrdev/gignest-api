use axum::{Json, Router, extract::Extension, routing::post};
use utils::{db::DB, error::AppError};
use authentication::register::Register;

pub async fn register(
    Extension(db): Extension<DB>,
    Json(form): Json<Register>,
) -> Result<(), AppError> {
    form.register(db).await
}

pub fn router() -> Router {
    Router::new().route("/register", post(register))
}
