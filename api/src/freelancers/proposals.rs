use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post},
};
use freelancers::proposals::{Proposal, ProposalID};
use sqlx::PgPool;
use utils::error::AppError;

pub async fn generate(
    Extension(db): Extension<PgPool>,

    Json(form): Json<Proposal>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn find_by_id(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    let proposal = id.get_by_proposal_id(db).await?;
    Ok(Json(proposal))
}

pub async fn find_freelancer_proposal(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    let proposal = id.get_by_freelancer_id(db).await?;
    Ok(Json(proposal))
}

pub async fn find_by_job_id(
    Path(id): Path<ProposalID>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Proposal>, AppError> {
    let proposal = id.get_by_job_id(db).await?;
    Ok(Json(proposal))
}

pub fn router() -> Router {
    Router::new()
        .route("/proposal", post(generate))
        .route("/{id}/proposal", get(find_freelancer_proposal))
        .route("/job/{id}/proposal", get(find_by_job_id))
        .route("/proposal/{id}", get(find_by_id))
}
