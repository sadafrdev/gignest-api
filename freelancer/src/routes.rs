use crate::features::{certificates, educations, languages, skills};
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificates::router(state.clone()))
        .merge(skills::router(state.clone()))
        .merge(languages::router(state.clone()))
        .merge(educations::router(state.clone()))
        .layer(Extension(state))
}
