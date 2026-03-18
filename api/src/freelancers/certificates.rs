use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, patch, post},
};
use utils::{db::AppState, error::AppError};
use freelancers::certificates::{Certificate, UpdateCertificate};

pub async fn create_certificate(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Certificate>,
) -> Result<(), AppError> {
    form.generate(db).await?;
}

pub async fn get_certificates(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    Certificate::get(db).await
}

pub async fn update_certificate(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdateCertificate>,
) -> Result<(), AppError> {
    form.update(db).await?;
    Ok(())
}

pub async fn delete_certificate(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), AppError> {
    form.delete(db).await?;
    Ok(())
}

pub fn router() -> Router {
    Router::new()
        .route("/certificates/{id}", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate/{id}", delete(delete_certificate))
}
