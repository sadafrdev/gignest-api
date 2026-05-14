use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{delete, get, post, put},
};
use clients::jobs::{Job, UpdateJob};
use utils::{db::DB, error::AppError};

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
    form.create(db).await
}

pub async fn update_job(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateJob>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_job(Path(id): Path<i64>, Extension(db): Extension<DB>) -> Result<(), AppError> {
    Job::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/job", put(update_job))
        .route("/job", delete(delete_job))
}
