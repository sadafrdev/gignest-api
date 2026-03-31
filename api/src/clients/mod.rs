use axum::Router;
use axum::middleware;
use utils::middleware::verify_token;
pub mod jobs;
pub mod contracts;

pub fn router() -> Router {
    let clients_routes = Router::new()
        .merge(jobs::router())
        .merge(contracts::router())
        .layer(middleware::from_fn(verify_token));

    Router::new().nest("/client", clients_routes)
}
