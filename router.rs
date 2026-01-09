use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/auth", auth::routes::router())
}
