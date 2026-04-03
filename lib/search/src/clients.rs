use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use utils::{db::DB, error::AppError};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Clients {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct SearchClients {
    pub title: Option<String>,
    pub budget_min: Option<BigDecimal>,
    pub budget_max: Option<BigDecimal>,
    pub job_type: Option<String>,
}

impl SearchClients{
    pub async fn search(self, db: DB) -> Result<Vec<Clients>, AppError> {
        let record = sqlx::query_as!(
            Clients,
            r#"
                SELECT DISTINCT 
                    u.id,
                    u.username,
                    u.email,
                    u.phone_number AS phone
                FROM users u
                LEFT JOIN jobs j ON j.client_id = u.id
                WHERE 
                    u.role = 'Client'
                    AND ($1::text IS NOT NULL OR title::text ILIKE $1)
                    AND ($2::NUMERIC IS NOT NULL OR budget_min >= $2)
                    AND ($3::NUMERIC IS NOT NULL OR budget_max <= $3)
                    AND ($4::text IS NOT NULL OR job_type::text = $4)
            "#,
            self.title,
            self.budget_min,
            self.budget_max,
            self.job_type
        )
        .fetch_all(&db)
        .await?;

        Ok(record)
    }
}