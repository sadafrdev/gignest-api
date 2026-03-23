use axum::{Extension, Router, middleware};
use utils::middleware::from_func;
use sqlx::PgPool;
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: PgPool) -> Router {
    Router::new()
        .merge(freelancers::router())
        .merge(authentication::router())
        .merge(clients::router())
        .layer(Extension(state))
        .layer(middleware::from_fn(from_func))
}
