use axum::{Router, routing::{get, post}};
pub mod review;

pub fn router() -> Router {
    Router::new()
    .route("/{reviewee_id}", get(review::find))
    .route("/", post(review::create))
}