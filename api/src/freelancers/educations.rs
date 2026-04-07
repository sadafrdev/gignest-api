use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use freelancers::educations::{Education, UpdateEducation};
use utils::{db::DB, error::AppError};

pub async fn create_education(
    Extension(db): Extension<DB>,
    Json(form): Json<Education>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn get_educations(
    Path(user_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Education>>, AppError> {
    Education::get(db, user_id).await.map(Json)
}

pub async fn update_education(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateEducation>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_education(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Education::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations/{id}", get(get_educations))
        .route("/education", put(update_education))
        .route("/education/{id}", delete(delete_education))
}
