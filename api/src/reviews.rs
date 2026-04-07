use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{get, post},
};
use utils::{db::DB, error::AppError};
use reviews::review::Review;

pub async fn create(
    Extension(db): Extension<DB>,
    Json(form): Json<Review>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn find(
    Path(reviewee_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Review>>, AppError> {
    Review::find_reviews_by_reviewee_id(reviewee_id, db)
        .await
        .map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/{reviewee_id}", get(find))
        .route("/", post(create))
}
