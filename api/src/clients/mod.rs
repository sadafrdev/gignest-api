use axum::Router;
pub mod jobs;

pub fn router() -> Router {
    let clients_routes = Router::new().merge(jobs::router());
    Router::new().nest("/client", clients_routes)
}
