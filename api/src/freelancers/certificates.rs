use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use utils::{db::DB, error::AppError};
use freelancers::certificates::{self, Certificate, UpdateCertificate, User, CertificateDelete};

pub async fn create_certificate(
    Extension(db): Extension<DB>,
    Json(form): Json<Certificate>,
) -> Result<(), AppError> {
    form.generate(db).await
}

pub async fn get_certificates(
    Path(user_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    Certificate::get(db, user_id).await
}

pub async fn update_certificate(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateCertificate>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_certificate(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    certificates::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/certificates/{id}", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/certificate", put(update_certificate))
        .route("/certificate/{id}", delete(delete_certificate))
}
