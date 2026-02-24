use axum::{Json, extract::Extension, http::StatusCode};
use serde::Deserialize;
use utils::db::AppState;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use rand_core::OsRng;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), StatusCode> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let res = sqlx::query(
            r#"
                SELECT
                    password, email
                FROM users
                WHERE email = $1 AND password = $2
            "#,
        )
        .bind(payload.email)
        .bind(hashed_password)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        match res {
            Some(_) => Ok(()),
            None => Err(StatusCode::NOT_FOUND),
        }
    }
}
