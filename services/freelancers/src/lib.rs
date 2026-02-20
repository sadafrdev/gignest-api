use axum::{Extension, Router};
use utils::db::AppState;
pub mod certificates;
pub mod educations;
pub mod languages;
pub mod skills;
// pub use lib_utils::db::establish_connection;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificates::router(state.clone()))
        .merge(skills::router(state.clone()))
        .merge(languages::router(state.clone()))
        .merge(educations::router(state.clone()))
        .layer(Extension(state))
}
