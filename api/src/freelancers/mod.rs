pub mod certificates;
pub mod educations;
pub mod languages;
pub mod proposals;
pub mod skills;
use axum::Router;

pub fn router() -> Router {
    let freelancer_routes = Router::new()
        .merge(certificates::router())
        .merge(skills::router())
        .merge(languages::router())
        .merge(educations::router())
        .merge(proposals::router());
    Router::new().nest("/freelancer", freelancer_routes)
}
