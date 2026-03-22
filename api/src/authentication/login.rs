use axum::{Json, Router, extract::Extension, routing::get};
use utils::{error::AppError, db::DB};
use authentication::login::Login;

pub async fn login(
    Extension(db): Extension<DB>,
    Json(form): Json<Login>,
) -> Result<(), AppError> {
    form.login(db).await
}

pub fn router() -> Router {
    Router::new().route("/login", get(login))
}
