use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use freelancers::languages::{self, Language, UpdateLanguage};
use utils::{db::DB, error::AppError};

pub async fn create_language(
    Extension(db): Extension<DB>,
    Json(form): Json<Language>,
) -> Result<(), AppError> {
    form.add(db).await
}

pub async fn get_languages(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Language>>, AppError> {
    Language::fetch(id, db).await.map(Json)
}

pub async fn update_language(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateLanguage>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_language(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    languages::delete(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/language/{id}", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/language/{id}", put(update_language))
}
