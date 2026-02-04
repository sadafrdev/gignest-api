pub use crate::certificate;
use crate::skills_routes;
pub use crate::languages;
use crate::languages_router;
use crate::educations;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(skills_routes(state.clone()))
        .merge(languages_router(state.clone()))
        .merge(educations::router(state.clone()))
        .layer(Extension(state))
}
