use authentication::login::{Login, LoginResponse};
use axum::{Extension, Json, Router, routing::post};
use utils::{db::DB, error::AppError};

pub async fn login(
    Extension(db): Extension<DB>,
    Json(form): Json<Login>,
) -> Result<Json<LoginResponse>, AppError> {
    form.login(db).await.map(Json)
}

pub fn router() -> Router {
    Router::new().route("/login", post(login))
}
