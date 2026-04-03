pub mod freelancers;
use axum::{Router, middleware};
use utils::middleware::from_func;

pub fn router() -> Router {
    Router::new()
        .merge(freelancers::router())
        .layer(middleware::from_fn(from_func))
}
