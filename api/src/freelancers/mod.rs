use axum::Router;
pub mod certificates;
pub mod educations;
pub mod languages;
pub mod proposals;
pub mod skills;
pub mod attachments;

pub fn router() -> Router {
    Router::new()
        .merge(certificates::router())
        .merge(skills::router())
        .merge(languages::router())
        .merge(educations::router())
        .merge(proposals::router())
        .merge(attachments::router())
}
