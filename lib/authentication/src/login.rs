use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError};
use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::PasswordHash;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
       self, db: DB
    ) -> Result<(), AppError>{
        
        let res = query!(
            "
                SELECT
                   password
                FROM users
                WHERE email = $1 
            ",
            self.email
        )
        .fetch_one(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        let parsed_hash = PasswordHash::new(&res.password)
        .map_err(|_| AppError::InternalServerError)?;

        Argon2::default()    
        .verify_password(self.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::ValidationError("Invalid credentials".to_string()))?;

        Ok(())
    }
}
