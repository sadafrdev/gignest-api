use axum::Extension;
use lib::AppState;

use crate::handlers::educations::{DeleteEducation, Education, UpdateEducation, User};
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
pub async fn create_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Education>,
) -> Result<(), StatusCode> {
    Education::create(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_educations(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Education>>, StatusCode> {
    let educations = Education::get(Extension(state), Json(payload)).await?;
    Ok(educations)
}

pub async fn update_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateEducation>,
) -> Result<(), StatusCode> {
    Education::update(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn delete_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<DeleteEducation>,
) -> Result<(), StatusCode> {
    Education::delete(Extension(state), Json(payload)).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations", get(get_educations))
        .route("/update-education", patch(update_education))
        .route("/delete-education", delete(delete_education))
        .layer(Extension(state))
}
