use bigdecimal::BigDecimal;
use core::str;
use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Job {
    pub client_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

impl Job {
    pub async fn create_job(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            " INSERT INTO jobs (client_id, title, description, budget_min, budget_max) VALUES ($1, $2, $3, $4, $5)",
            self.client_id,
            self.title,
            self.description,
            self.budget_min,
            self.budget_max
        )
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
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

impl UpdateJob{
    pub async fn update_job(
       self, db: DB
    ) -> Result<(), AppError> {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct JobID {
    id: i64,
}

impl JobID {
    pub async fn delete_job(
        self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            " DELETE FROM jobs WHERE id = $1 ",
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub client_id: i64,
}

impl Client {
    pub async fn find(self,  db: DB ) -> Result<Job, AppError> {
        sqlx::query_as!(
            Job,
            "
                SELECT
                    client_id,
                    title,
                    description,
                    budget_min,
                    budget_max
                FROM jobs
                WHERE client_id = $1
            ",
            self.client_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::NotFound("JOB"))
    }
}