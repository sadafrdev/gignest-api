use axum::{Json, routing::{delete, get, post, put}, extract::Extension, Router};
use utils::{db::DB, error::AppError};
use clients::jobs::{Client, Job, JobID, UpdateJob};

pub async fn get_jobs(
    Extension(db): Extension<DB>,
    Json(form): Json<Client>,
) -> Result<Json<Job>, AppError> {
    form.find(db).await.map(Json)
}

pub async fn create_job(
    Extension(db): Extension<DB>,
    Json(form): Json<Job>,
) -> Result<(), AppError> {
    form.create_job(db).await
}

pub async fn update_job(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateJob>,
) -> Result<(), AppError> {
    form.update_job(db).await
}

pub async fn delete_job(
    Extension(db): Extension<DB>,
    Json(form): Json<JobID>,
) -> Result<(), AppError> {
    form.delete_job(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/update-job", put(update_job))
        .route("/delete-job", delete(delete_job))
}
