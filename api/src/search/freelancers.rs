use axum::{
    Extension, Json, Router, routing::get
};
use utils::{db::DB, error::AppError};
use search::freelancers::{Freelancer, SearchFreelancer};

pub async fn search_freelancer(
    Extension(db): Extension<DB>,
    Json(form): Json<SearchFreelancer>,
) -> Result<Json<Vec<Freelancer>>, AppError> {
    form.search(db).await.map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/freelancer/search", get(search_freelancer))
}
