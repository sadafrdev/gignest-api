use axum::{Extension, Router};
use libs::AppState;
pub mod certificates;
pub mod educations;
pub mod languages;
pub mod skills;
pub use libs::utils::db::establish_connection;
pub use libs::clients;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificates::router(state.clone()))
        .merge(skills::router(state.clone()))
        .merge(languages::router(state.clone()))
        .merge(educations::router(state.clone()))
        .layer(Extension(state))
}
