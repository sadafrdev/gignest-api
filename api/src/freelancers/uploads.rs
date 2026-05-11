use axum::{Json, Router, extract::{Extension, Multipart, Path}, response::IntoResponse, routing::{delete, get, post}};
use utils::{db::DB, error::AppError};
use freelancers::uploads::Uploads;

pub async fn get_uploads(
    Path(user_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Uploads>>, AppError> {
    Uploads::get_uploads(db, user_id).await.map(Json)
}

pub async fn download_upload(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<impl IntoResponse, AppError> {
    Uploads::download(id, db).await
}

pub async fn create(
    Extension(db): Extension<DB>,
    multi: Multipart,
) -> Result<(), AppError> {
    Uploads::create(multi, db).await
}

pub async fn delete_upload(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Uploads::delete(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/uploads", post(create))
        .route("/upload/{id}", get(download_upload))
        .route("/upload/{id}", delete(delete_upload))
        .route("/uploads/{user_id}", get(get_uploads))
}
