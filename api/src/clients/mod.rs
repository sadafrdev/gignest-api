use axum::Router;
use axum::middleware;
use utils::middleware::{verify_token, verify_role};
pub mod jobs;
pub mod contracts;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
        .merge(contracts::router())
        .layer(middleware::from_fn(verify_role))
        .layer(middleware::from_fn(verify_token))
}
