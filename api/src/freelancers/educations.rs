use axum::{
    Extension, extract::Path,
    Json, Router,
    routing::{delete, get, patch, post},
};
use utils::{db::DB, error::AppError};
use freelancers::educations::{Education, UpdateEducation, EducationID};

pub async fn create_education(
    Extension(db): Extension<DB>,
    Json(form): Json<Education>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn get_educations(
    Path(id): Path<EducationID>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Education>>, AppError> {
    id.get(db).await
}

pub async fn update_education(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateEducation>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_education(
    Path(id): Path<EducationID>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    id.delete(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations/{id}", get(get_educations))
        .route("/update-education", patch(update_education))
        .route("/delete-education/{id}", delete(delete_education))
}
