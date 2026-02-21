use axum::{Extension, Router};
use utils::db::AppState;
pub mod jobs;

pub fn router(state: AppState) -> Router {
    let clients_routes = Router::new()
        .merge(jobs::router(state.clone()))
        .layer(Extension(state));
    Router::new().nest("/client", clients_routes)

}
