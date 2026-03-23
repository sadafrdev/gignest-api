use axum::{Extension, Router};
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
}
