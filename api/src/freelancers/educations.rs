use axum::{Extension, extract::Path};
use utils::db::AppState;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use freelancers::educations::{Education, UpdateEducation};

pub async fn create_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Education>,
) -> Result<(), StatusCode> {
    Education::create(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_educations(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Education>>, StatusCode> {
    let educations = Education::get(Extension(state), id).await?;
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
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    Education::delete(Extension(state), id).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations/{id}", get(get_educations))
        .route("/update-education", patch(update_education))
        .route("/delete-education/{id}", delete(delete_education))
        .layer(Extension(state))
}
