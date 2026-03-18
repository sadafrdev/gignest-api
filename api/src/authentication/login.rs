use axum::{Json, Router, extract::Extension, routing::get};
use sqlx::PgPool;
use utils::error::AppError;
use authentication::login::Login;

pub async fn login(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Login>,
) -> Result<(), AppError> {
    form.login(db).await
}

pub fn router() -> Router {
    Router::new().route("/login", get(login))
}
