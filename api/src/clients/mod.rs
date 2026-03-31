use axum::Router;
use axum::middleware;
use utils::middleware::from_func;
pub mod jobs;
pub mod reviews;

pub fn router() -> Router {
    let clients_routes = Router::new()
        .merge(jobs::router())
        .layer(middleware::from_fn(from_func));

    Router::new().nest("/client", clients_routes)
}
