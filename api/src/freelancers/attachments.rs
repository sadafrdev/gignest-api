use axum::{Json, Router, extract::{Extension, Multipart, Path}, response::IntoResponse, routing::{delete, get, post}};
use utils::{db::DB, error::AppError};
use freelancers::attachments::Attachment;

pub async fn get_attachments(
    Path(user_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<attachment>>, AppError> {
    Attachment::get(db, user_id).await.map(Json)
}

pub async fn download_attachment(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<impl IntoResponse, AppError> {
    Attachment::download(id, db).await
}

pub async fn create(
    Extension(db): Extension<DB>,
    multi: Multipart,
) -> Result<(), AppError> {
    Attachment::create(multi, db).await
}

pub async fn delete_attachment(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Attachment::delete(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/attachments", post(create))
        .route("/attachment/{id}", get(download_attachment))
        .route("/attachment/{id}", delete(delete_attachment))
        .route("/attachments/{user_id}", get(get_attachments))
}
