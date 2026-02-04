use axum::Json;

pub mod jobs;
pub mod routes;
use axum::{extract::Extension, http::StatusCode};
use lib::AppState;
use jobs::{ClientID, UpdateJob, JobID};
use axum::routing::{get, post, put, delete};
use jobs::Job;

pub async fn get_jobs(
    Extension(state): Extension<AppState>,
    Json(payload): Json<ClientID>,
) -> Result<Json<Option<Job>>, StatusCode> {
    let jobs = Job::get_jobs(Extension(state), Json(payload)).await?;
    Ok(Json(jobs))
}

pub async fn create_job(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Job>,
) -> Result<(), StatusCode> {
    Job::create_job(Extension(state), Json(payload)).await
}

pub async fn update_job(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateJob>,
) -> Result<(), StatusCode> {
    Job::update_job(Extension(state), Json(payload)).await
}

pub async fn delete_job(
    Extension(state): Extension<AppState>,
    Json(payload): Json<JobID>,
) -> Result<(), StatusCode> {
    println!("Deleted job with ID:");
    Job::delete_job(Extension(state), Json(payload)).await
}

pub fn job_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/update-job", put(update_job))
        .route("/delete-job", delete(delete_job))
        .layer(Extension(state))
}