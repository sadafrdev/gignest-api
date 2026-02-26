use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utils::enums::Country;
use utils::{db::DB, error::AppError};
use validator::Validate;

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
    pub async fn register(self, db: DB) -> Result<(), AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

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
            AppError::InternalServerError
        })?;

        Ok(())
    }
}
