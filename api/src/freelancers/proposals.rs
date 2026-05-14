use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post},
};
use freelancers::proposals::Proposal;
use utils::{db::DB, error::AppError};

pub async fn create(
    Extension(db): Extension<DB>,
    Json(form): Json<Proposal>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn find_by_id(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Proposal>, AppError> {
    Proposal::get_by_proposal_id(db, id).await.map(Json)
}

pub async fn find_proposal(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Proposal>, AppError> {
    Proposal::get_by_freelancer_id(db, id).await.map(Json)
}

pub async fn find_by_job_id(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Proposal>, AppError> {
    Proposal::get_by_job_id(db, id).await.map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/proposal", post(create))
        .route("/{id}/proposal", get(find_proposal))
        .route("/job/{id}/proposal", get(find_by_job_id))
        .route("/proposal/{id}", get(find_by_id))
}
