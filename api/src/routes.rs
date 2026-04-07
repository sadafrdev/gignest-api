use axum::{Extension, Router};
use utils::{db::DB, middleware::{verify_token, verify_role}};
use crate::reviews;
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: DB) -> Router {
    let routes = Router::new()
        .nest("/freelancer", freelancers::router())
        .nest("/client", clients::router())
        .nest("/reviews", reviews::router())
        .layer(middleware::from_fn(verify_token))
        .layer(middleware::from_fn(verify_role));

    Router::new()
        .merge(authentication::router())
        .merge(routes)
        .layer(Extension(state))
}
