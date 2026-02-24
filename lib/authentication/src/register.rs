use axum::{Json, extract::Extension, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utils::db::AppState;
use utils::enums::Country;
use validator::Validate;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use rand_core::OsRng;

#[derive(Deserialize, Serialize, Debug, FromRow, Validate)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub phone_number: String,
    pub username: String,
    pub country: Country,
    pub role: Role,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Freelancer,
    Client,
}

impl Register {
    pub async fn register(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), StatusCode> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        
        sqlx::query(
            "
            INSERT INTO users
            (first_name, last_name, password, email, phone_number, username, country, role)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
        )
        .bind(payload.first_name)
        .bind(payload.last_name)
        .bind(hashed_password)
        .bind(payload.email)
        .bind(payload.phone_number)
        .bind(payload.username)
        .bind(payload.country as Country)
        .bind(payload.role as Role)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
}
