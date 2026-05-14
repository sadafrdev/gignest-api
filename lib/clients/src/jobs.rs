use bigdecimal::BigDecimal;
use core::str;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, enums::JobType, error::AppError};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Type)]
pub struct Job {
    pub client_id: Option<i64>,
    pub title: String,
    pub job_type: JobType,
    pub description: String,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

impl Job {
    pub async fn create(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            " 
                INSERT INTO jobs (client_id, title, description, job_type , budget_min, budget_max) 
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
            self.client_id,
            self.title,
            self.description,
            self.job_type as JobType,
            self.budget_min,
            self.budget_max
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn find(db: DB, client_id: i64) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            r#"
                SELECT
                    client_id,
                    title,
                    description,
                    job_type AS "job_type: JobType", 
                    budget_min,
                    budget_max
                FROM jobs
                WHERE client_id = $1
            "#,
            client_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::NotFound("JOB".to_string()))
    }

    pub async fn delete(db: DB, id: i64) -> Result<(), AppError> {
        sqlx::query!(" DELETE FROM jobs WHERE id = $1 ", id)
            .execute(&db)
            .await?;
    
        Ok(())
    }

}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateJob {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub job_type: JobType,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

impl UpdateJob {
    pub async fn update(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE jobs
                SET
                    title = $1,
                    description = $2,
                    budget_min = $3,
                    budget_max = $4
                WHERE id = $5
            ",
            self.title,
            self.description,
            self.budget_min,
            self.budget_max,
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }

}
