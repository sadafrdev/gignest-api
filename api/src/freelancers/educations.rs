use axum::{
    Extension, extract::Path,
    Json, Router,
    routing::{delete, get, patch, post},
};
use sqlx::PgPool;
use utils::{db::AppState, error::AppError};
use freelancers::educations::{Education, UpdateEducation, DeleteEducation};

pub async fn create_education(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Education>,
) -> Result<(), AppError> {
    form.create(db).await?
}

pub async fn get_educations(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Education>>, AppError> {
    Education::get(db).await.map(Json).map(Json)
}

pub async fn update_education(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdateEducation>,
) -> Result<(), AppError> {
    form.update(Extension(db)).await?
}

pub async fn delete_education(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), AppError> {
    DeleteEducation::delete(db).await?
}

pub fn router() -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations/{id}", get(get_educations))
        .route("/update-education", patch(update_education))
        .route("/delete-education/{id}", delete(delete_education))
}
