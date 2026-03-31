use axum::Router;
use axum::middleware;
use utils::middleware::{from_func, verify_role};
pub mod jobs;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
        .layer(middleware::from_fn(verify_role))
        .layer(middleware::from_fn(from_func))
}
