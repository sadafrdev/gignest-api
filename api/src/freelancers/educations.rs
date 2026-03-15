use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use freelancers::educations::{Education, UpdateEducation};
use sqlx::PgPool;

pub async fn create_education(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Education>,
) -> Result<(), StatusCode> {
    Education::create(db, Json(payload)).await?;
    Ok(())
}

pub async fn get_educations(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Education>>, StatusCode> {
    let educations = Education::get(db, id).await?;
    Ok(educations)
}

pub async fn update_education(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<UpdateEducation>,
) -> Result<(), StatusCode> {
    Education::update(db, Json(payload)).await?;
    Ok(())
}

pub async fn delete_education(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), StatusCode> {
    Education::delete(db, id).await?;
    Ok(())
}

pub fn router() -> Router {
    Router::new()
        .route("/education", post(create_education))
        .route("/educations/{id}", get(get_educations))
        .route("/update-education", patch(update_education))
        .route("/delete-education/{id}", delete(delete_education))
}
