use axum::{Extension, extract::Path, routing::{Router, post}};
use utils::{db::DB, error::AppError};
use clients::reviews::{self, Review};

pub async  fn create_review(
    Extension(db): Extension<DB>,
    Extension(form): Extension<Review>,
) -> Result<(), AppError> {
    form.create_review(db).await
}

pub async fn get_reviews_by_reviewee_id(
    Path(reviewee_id): Path<i64>,
    Extension(db): Extension<DB>
) -> Result<Json<Vec<Review>>, AppError> {
    reviews::get_reviews_by_reviewee_id(reviewee_id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/reviews", post(create_review))
        .route("/reviews/{reviewee_id}", get(get_reviews_by_reviewee_id))
}