pub use crate::certificate;
use crate::skills_routes;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(skills_routes(state.clone()))
        .layer(Extension(state))
}
