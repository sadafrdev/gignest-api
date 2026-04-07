use serde::{Deserialize, Serialize};
use utils::{db::DB, enums::Country, error::AppError};

#[derive(Deserialize, Serialize, Debug)]
pub struct Freelancer {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<Country>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchFreelancerParam {
    pub skill: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
}

impl Freelancer {
    pub async fn search(db: &DB, params: SearchFreelancerParam) -> Result<Vec<Self>, AppError> {
        let record = sqlx::query_as!(
            Self,
            r#"
                SELECT DISTINCT 
                    U.id AS user_id,
                    U.first_name,
                    U.last_name,
                    U.country AS "country: Country"
                FROM users U
                LEFT JOIN skills S ON S.user_id = U.id
                LEFT JOIN languages L ON L.user_id = U.id
                WHERE 
                    U.role = 'Freelancer' 
                    AND ($1::text IS NULL OR S.skill::text ILIKE $1)
                    AND ($2::text IS NULL OR L.language::text ILIKE $2)
                    AND ($3::text IS NULL OR U.country::text = $3)
            "#,
            params.skill,
            params.language,
            params.country
        )
        .fetch_all(db)
        .await?;

        Ok(record)
    }
}
