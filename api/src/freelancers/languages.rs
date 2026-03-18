use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, patch, post},
};
use sqlx::PgPool;
use utils::{db::AppState, error::AppError};
use freelancers::languages::{Language, UpdateLanguage, User};

pub async fn create_language(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Language>,
) -> Result<(), AppError> {
    form.add(db).await?
}

pub async fn get_languages(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Language>>, AppError> {
    Language::get(state.db).await.map(Json)
}

pub async fn update_language(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdateLanguage>,
) -> Result<(), AppError> {
    form.update(db).await?
}

pub async fn delete_language(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), AppError> {
    User::delete(db).await?;
}

pub fn router() -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/delete-language/{id}", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/update-language/{id}", patch(update_language))
}
