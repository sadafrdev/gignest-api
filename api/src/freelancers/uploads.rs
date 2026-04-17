use axum::{Router, extract::{Extension, Multipart}, routing::post};
use utils::{db::DB, error::AppError};
use freelancers::uploads;

pub async fn create(
    Extension(db): Extension<DB>,
    multi: Multipart,
) -> Result<(), AppError> {
    uploads::create(multi, db).await
}

pub fn router() -> Router {
    Router::new().route("/uploads", post(create))
}
