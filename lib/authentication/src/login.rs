use axum::{Json, extract::Extension};
use serde::Deserialize;
use utils::db::AppState;
use utils::error::AppError;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), AppError> {
        let res = sqlx::query(
            r#"
                SELECT
                    password, email
                FROM users
                WHERE email = $1 AND password = $2
            "#,
        )
        .bind(payload.email)
        .bind(payload.password)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        match res {
            Some(_) => Ok(()),
            None => Err(AppError::NotFound("USER".to_string())),
        }
    }
}
