use axum::{Json, Router, extract::Extension, routing::get};
use utils::{db::DB, error::AppError};
use authentication::login::{Login, LoginResponse};

pub async fn login(
    Extension(db): Extension<DB>,
    Json(form): Json<Login>,
) -> Result<Json<LoginResponse>, AppError> {
    form.login(db).await

}

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
}
