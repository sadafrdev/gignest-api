use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post},
};
use sqlx::PgPool;
use utils::error::AppError;
use freelancers::proposals::{Proposal, ProposalID};

pub async fn create(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Proposal>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn find_by_id(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    id.get_by_proposal_id(db).await.map(Json)
}

pub async fn find_proposal(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    id.get_by_freelancer_id(db).await.map(Json)
}

pub async fn find_by_job_id(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    id.get_by_job_id(db).await.map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/proposal", post(create))
        .route("/{id}/proposal", get(find_proposal))
        .route("/job/{id}/proposal", get(find_by_job_id))
        .route("/proposal/{id}", get(find_by_id))
}
