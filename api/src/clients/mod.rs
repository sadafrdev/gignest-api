use axum::Router;
pub mod jobs;
pub mod contracts;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
        .merge(contracts::router())
}
