use axum::Json;
use axum::http::StatusCode;
use bigdecimal::BigDecimal;
use core::str;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Job {
    pub client_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateJob {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JobID {
    id: i64,
}

impl Job {
    pub async fn create_job(db: PgPool, Json(payload): Json<Job>) -> Result<(), StatusCode> {
        sqlx::query(
            "
                INSERT INTO jobs (client_id, title, description, budget_min, budget_max)
                VALUES ($1, $2, $3, $4, $5)
            ",
        )
        .bind(payload.client_id)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(payload.budget_min)
        .bind(payload.budget_max)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }

    pub async fn update_job(db: PgPool, Json(payload): Json<UpdateJob>) -> Result<(), StatusCode> {
        sqlx::query(
            "
                UPDATE jobs
                SET
                    title = $1,
                    description = $2,
                    budget_min = $3,
                    budget_max = $4
                WHERE id = $5
            ",
        )
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(payload.budget_min)
        .bind(payload.budget_max)
        .bind(payload.id)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });

        Ok(())
    }

    pub async fn delete_job(db: PgPool, Json(payload): Json<JobID>) -> Result<(), StatusCode> {
        sqlx::query(" DELETE FROM jobs WHERE id = $1 ")
            .bind(payload.id)
            .execute(&db)
            .await
            .map_err(|e| {
                eprintln!("SQL ERROR: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Jobs {
    pub client_id: i64,
}
impl Jobs {
    pub async fn find(
        db: PgPool,
        Json(payload): Json<Self>,
    ) -> Result<Option<Job>, StatusCode> {
        let jobs = sqlx::query_as!(
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
            payload.client_id
        )
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(jobs)
    }
}