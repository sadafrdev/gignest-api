use axum::{Extension, extract::Path};
use axum::{
    Json, Router,
    routing::{delete, get, patch, post},
};
use freelancers::educations::{Education, UpdateEducation};
use utils::{db::AppState, error::AppError};

pub async fn create_education(
    Extension(state): Extension<AppState>,
    Json(form): Json<Education>,
) -> Result<(), AppError> {
    form.create(state.db).await?;
    Ok(())
}

pub async fn get_educations(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Education>>, AppError> {
    let educations = Education.get(state.db).await?;
    Ok(educations)
}

pub async fn update_education(
    Extension(state): Extension<AppState>,
    Json(form): Json<UpdateEducation>,
) -> Result<(), AppError> {
    form.update(Extension(state.db)).await?;
    Ok(())
}

pub async fn delete_education(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    Education.delete(state.db).await?;
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
