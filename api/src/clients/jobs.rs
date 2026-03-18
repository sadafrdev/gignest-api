use axum::{
    Json, Router,
    extract::Extension,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use clients::jobs::{Jobs, Job, JobID, UpdateJob};

pub async fn get_jobs(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Jobs>,
) -> Result<Json<Option<Job>>, StatusCode> {
    Jobs::find(db, Json(payload)).await.map(Json)
}

pub async fn create_job(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Job>,
) -> Result<(), StatusCode> {
    Job::create_job(db, Json(payload)).await
}

pub async fn update_job(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<UpdateJob>,
) -> Result<(), StatusCode> {
    Job::update_job(db, Json(payload)).await
}

pub async fn delete_job(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<JobID>,
) -> Result<(), StatusCode> {
    Job::delete_job(db, Json(payload)).await
}

pub fn router() -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/update-job", put(update_job))
        .route("/delete-job", delete(delete_job))
}
