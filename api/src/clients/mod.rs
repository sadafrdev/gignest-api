use axum::Router;
use axum::middleware;
use utils::middleware::{from_func, role_middleware};
pub mod jobs;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
        .layer(middleware::from_fn(role_middleware))
        .layer(middleware::from_fn(from_func))
}
