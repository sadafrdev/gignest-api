use axum::Router;
pub mod contracts;
pub mod jobs;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
        .merge(contracts::router())
}
