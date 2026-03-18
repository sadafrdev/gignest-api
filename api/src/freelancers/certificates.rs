use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use sqlx::PgPool;
use freelancers::certificates::Certificate;

pub async fn create_certificate(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    Certificate::generate(db, Json(payload)).await?;
    Ok(())
}

pub async fn get_certificates(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Certificate>>, StatusCode> {
    Certificate::get(db, id).await
}

pub async fn update_certificate(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    Certificate::update(db, Json(payload)).await?;
    Ok(())
}

pub async fn delete_certificate(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), StatusCode> {
    Certificate::delete(db, id).await?;
    Ok(())
}

pub fn router() -> Router {
    Router::new()
        .route("/certificates/{id}", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate/{id}", delete(delete_certificate))
}
