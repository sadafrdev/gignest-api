use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use serde::Serialize;
use sqlx::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("Invalid input data: {0}")]
    ValidationError(&'static str),
    #[error("Resource not found: {0}")]
    NotFound(&'static str),
    #[error("Database error: {0}")]
    DatabaseError(#[from] Error), 
    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Serialize)]
pub struct ErrorResponse{
    message: &'static str
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        let ( status, error_message) = match self {
            AppError::ValidationError(msg)=> (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(err) => {
                eprintln!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed")
            },
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred"),
        };

        let body = Json(ErrorResponse{
            message: error_message
        });

        (status, body).into_response()
    }
}