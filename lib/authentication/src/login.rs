use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError, jwt::create_jwt, encryption::verify_password};
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
            " SELECT id, password FROM users WHERE email = $1 ",
            self.email
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        verify_password(&res.password, self.password)?;
        
        let token = create_jwt(res.id)?;
        
        Ok(LoginResponse { token })
    }
}
