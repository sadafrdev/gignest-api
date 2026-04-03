use axum::{Extension, extract::Path, routing::{Router, post}};
use utils::{db::DB, error::AppError};
use reviews::find;

pub async fn find(
    Path(reviewee_id): Path<i64>,
    Extension(db): Extension<DB>
) -> Result<Json<Vec<Review>>, AppError> {
    find::find_reviews_by_reviewee_id(reviewee_id, db).await
}

pub fn router() -> Router {
    Router::new()
    .route("/{reviewee_id}", get(find))
}