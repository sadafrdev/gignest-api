
use axum::middleware;
use axum::Router;
use utils::middleware::{from_func, role_middleware};
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
        .layer(middleware::from_fn(role_middleware))
        .layer(middleware::from_fn(from_func))
}
