use axum::{Router, middleware};
use utils::middleware::from_func;
pub mod freelancers;
pub mod clients;

pub fn router() -> Router {
    Router::new()
        .merge(freelancers::router())
        .merge(clients::router())
        .layer(middleware::from_fn(from_func))
}
