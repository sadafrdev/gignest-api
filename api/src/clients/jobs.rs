use std::path::Path;

use axum::{Json, routing::{delete, get, post, put}, extract::Extension, Router};
use utils::{db::DB, error::AppError};
use clients::jobs::{self, Jobs, Job, JobID, UpdateJob};

pub async fn get_jobs(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Job>, AppError> {
    Job::find(db, id).await.map(Json)
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
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    jobs::delete_job(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/job", put(update_job))
        .route("/job", delete(delete_job))
}
