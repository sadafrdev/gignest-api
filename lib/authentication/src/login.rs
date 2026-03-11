use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError, jwt::create_jwt};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(self, db: DB) -> Result<LoginResponse, AppError> {
        let res = query!(
            "SELECT id, password FROM users WHERE email = $1",
            self.email
        )
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        let user = match res {
            Some(u) => u,
            None => return Err(AppError::NotFound("USER".to_string())),
        };
        if user.password != self.password {
            return Err(AppError::Unauthorized);
        }
        
        let token = create_jwt(user.id)
            .map_err(|_| AppError::InternalServerError)?;
        
        Ok(LoginResponse { token })
    }
}
