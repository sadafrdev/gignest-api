use axum::{http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utils::enums::Country;
use validator::Validate;
use utils::{db::DB, security::hash_password};

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role")]
pub enum Role {
    Freelancer,
    Client,
}

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

impl Register {
    pub async fn register(
       self, db: DB
    ) -> Result<(), StatusCode> {
        let hashed_password = hash_password(self.password).map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        sqlx::query!(
        "
            INSERT INTO users
            (first_name, last_name, password, email, phone_number, username, country, role)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
        self.first_name,
        self.last_name,
        hashed_password,
        self.email,
        self.phone_number,
        self.username,
        self.country as Country,
        self.role as Role
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
}
