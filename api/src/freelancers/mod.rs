
use axum::middleware;
use axum::Router;
use utils::middleware::{from_func, verify_role};
pub mod certificates;
pub mod educations;
pub mod languages;
pub mod proposals;
pub mod skills;

pub fn router() -> Router {
    Router::new()
        .merge(certificates::router())
        .merge(skills::router())
        .merge(languages::router())
        .merge(educations::router())
        .merge(proposals::router())
        .layer(middleware::from_fn(verify_role))
        .layer(middleware::from_fn(from_func))
}
