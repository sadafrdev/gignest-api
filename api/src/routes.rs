use axum::{Extension, Router, middleware};
use utils::{
    db::DB,
    middleware::{verify_role, verify_token},
};
use crate::reviews;
use crate::{authentication, clients, freelancers, search};

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
        .nest("/search", search::router())
        .layer(Extension(state))
}
