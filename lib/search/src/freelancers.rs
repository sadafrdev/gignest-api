use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, enums::Country, error::AppError};

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct Freelancer {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<Country>,
}

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct SearchFreelancer {
    pub skill: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>
}

impl SearchFreelancer {
    pub async fn search(self,  db: DB) -> Result<Vec<Freelancer>, AppError> {
        let record = sqlx::query_as!(
            Freelancer,
            r#"
                SELECT DISTINCT 
                    u.id AS user_id,
                    u.first_name,
                    u.last_name,
                    u.country AS "country: Country"
                FROM users u
                LEFT JOIN skills s ON s.user_id = u.id
                LEFT JOIN languages l ON l.user_id = u.id
                WHERE 
                    u.role = 'Freelancer' 
                    AND ($1::text IS NULL OR s.skill::text ILIKE $1)
                    AND ($2::text IS NULL OR l.language::text ILIKE $2)
                    AND ($3::text IS NULL OR u.country::text = $3)
            "#,
            self.skill,
            self.language,
            self.country
        )
        .fetch_all(&db)
        .await?;

        Ok(record)
    }
}