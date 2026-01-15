use axum::{
    Extension, Router
};
use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", authentication::routes::router(state.clone()))
        .nest("/freelancer", freelancer::routes::router(state.clone()))
        .layer(Extension(state))
}
