
use axum::{Extension, Router};
use lib::AppState;
use crate::features::{languages, skills, certificates};

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificates::router(state.clone()))
        .merge(skills::skills_routes(state.clone()))
        .merge(languages::languages_router(state.clone()))
        .layer(Extension(state))
}
