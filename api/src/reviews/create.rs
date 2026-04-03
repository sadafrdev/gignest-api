use axum::{Extension, extract::Path, routing::{Router, post}};
use utils::{db::DB, error::AppError};
use reviews::create::Review;

pub async fn create_review(
    Extension(db): Extension<DB>,
    Extension(form): Extension<Review>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_review))
}