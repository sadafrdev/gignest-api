use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
pub use freelancers::languages::Language;
use sqlx::PgPool;

pub async fn create_language(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Language>,
) -> Result<(), StatusCode> {
    Language::add(db, Json(payload)).await?;
    Ok(())
}

pub async fn get_languages(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Language>>, StatusCode> {
    let languages = Language::get(db, id).await?;
    Ok(languages)
}

pub async fn update_language(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Language>,
) -> Result<(), StatusCode> {
    Language::update(db, Json(payload)).await?;
    Ok(())
}

pub async fn delete_language(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), StatusCode> {
    Language::delete(db, id).await?;
    Ok(())
}

pub fn router() -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/delete-language/{id}", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/update-language/{id}", patch(update_language))
}
