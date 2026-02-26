use axum::Json;
use axum::routing::{delete, get, post, put};
use axum::extract::Extension;
use clients::jobs::{ClientID, Job, JobID, UpdateJob};
use utils::{db::AppState, error::AppError};

pub async fn get_jobs(
    Extension(state): Extension<AppState>,
    Json(form): Json<ClientID>,
) -> Result<Json<Option<Job>>, AppError> {
    let jobs = Job::get_jobs(state.db).await?;
    Ok(Json(jobs))
}

pub async fn create_job(
    Extension(state): Extension<AppState>,
    Json(form): Json<Job>,
) -> Result<(), AppError> {
    form.create_job(state.db).await
}

pub async fn update_job(
    Extension(state): Extension<AppState>,
    Json(form): Json<UpdateJob>,
) -> Result<(), AppError> {
    form.update_job(state.db).await
}

pub async fn delete_job(
    Extension(state): Extension<AppState>,
    Json(form): Json<JobID>,
) -> Result<(), AppError> {
    println!("Deleted job with ID:");
    UpdateJob::delete_job(state.db).await
}

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .route("/update-job", put(update_job))
        .route("/delete-job", delete(delete_job))
        .layer(Extension(state))
}
