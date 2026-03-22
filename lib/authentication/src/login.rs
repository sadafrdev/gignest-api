use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError, security::verify};


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

        verify(&res.password, self.password)?;
        
        Ok(())
    }
}
