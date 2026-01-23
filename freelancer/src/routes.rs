pub use crate::certificate;
use axum::{Extension, Router};
use lib::AppState;
use crate::skills_routes;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(skills_routes(state.clone()))
        .layer(Extension(state))
}
