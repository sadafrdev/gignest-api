use authentication::register::Register;
use axum::{Json, Router, extract::Extension, http::StatusCode, routing::post};
use sqlx::PgPool;

pub async fn register(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Register>,
) -> Result<(), StatusCode> {
    form.register(db).await
}

pub fn router() -> Router {
    Router::new().route("/register", post(register))
}
