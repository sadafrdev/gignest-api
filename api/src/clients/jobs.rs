use axum::{Json, routing::{delete, get, post, put}, extract::Extension, PgPool};
use utils::{db::AppState, error::AppError};
use clients::jobs::{ClientID, Job, JobID, UpdateJob};

pub async fn get_jobs(
    Extension(db): Extension<PgPool>,
    Json(form): Json<ClientID>,
) -> Result<Json<Option<Job>>, AppError> {
    Jobs::find(db).await.map(Json)
}

pub async fn create_job(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Job>,
) -> Result<(), AppError> {
    form.create_job(db).await
}

pub async fn update_job(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdateJob>,
) -> Result<(), AppError> {
    form.update_job(db).await
}

pub async fn delete_job(
    Extension(db): Extension<PgPool>,
    Json(form): Json<JobID>,
) -> Result<(), AppError> {
    UpdateJob::delete_job(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/update-job", put(update_job))
        .route("/delete-job", delete(delete_job))
}
