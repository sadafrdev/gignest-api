use axum::Json;
use axum::routing::{post, get};
use axum::extract::{Extension, Path};
use freelancers::proposals::{Proposal, ProposalID};
use utils::{error::AppError, db::AppState};

pub async fn generate(
    Extension(state): Extension<AppState>,
    Json(form): Json<Proposal>,
) -> Result<(), AppError> {
    form.create(state.db).await
}

pub async fn fetch_by_proposal_id(
    Extension(state): Extension<AppState>,
    Path(id): Path<ProposalID>,
) -> Result<Json<Option<Proposal>>, AppError> {
    let proposal = id.get_by_proposal_id(state.db).await?;
    Ok(Json(proposal))
}

pub async fn fetch_by_freelancer_id(
    Extension(state): Extension<AppState>,
    Path(id): Path<ProposalID>,
) -> Result<Json<Option<Proposal>>, AppError> {
    let proposal = id.get_by_freelancer_id(state.db).await?;
    Ok(Json(proposal))
}

pub async fn fetch_by_job_id(
    Extension(state): Extension<AppState>,
    Path(id): Path<ProposalID>,
) -> Result<Json<Option<Proposal>>, AppError> {
    let proposal = id.get_by_job_id(state.db).await?;
    Ok(Json(proposal))
}

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/proposal", post(generate))
        .route("/{id}/proposal", get(fetch_by_freelancer_id))
        .route("/job/{id}/proposal", get(fetch_by_job_id))
        .route("/proposal/{id}", get(fetch_by_proposal_id))
        .layer(Extension(state))
}
