use axum::Router;
pub mod jobs;

pub fn router() -> Router {
    Router::new()
        .merge(jobs::router())
}
