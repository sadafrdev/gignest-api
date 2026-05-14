use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{
    db::DB,
    enums::{JobType, ProposalStatus},
    error::AppError,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    freelancer_id: Option<i64>,
    cover_letter: String,
    job_id: Option<i64>,
    bid_amount: BigDecimal,
    job_type: JobType,
    status: ProposalStatus,
}

impl Proposal {
    pub async fn create(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                SELECT job_id
                FROM proposals
                WHERE job_id = $1 AND freelancer_id = $2
            ",
            self.job_id,
            self.freelancer_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::NotFound("FREELANCER".to_string()))?;

        sqlx::query!(
            " INSERT INTO proposals (freelancer_id, cover_letter, job_id, bid_amount, job_type, status) VALUES ($1, $2, $3, $4, $5, $6) ",
            self.freelancer_id,
            self.cover_letter,
            self.job_id,
            self.bid_amount,
            self.job_type as JobType,
            self.status as ProposalStatus
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn get_by_proposal_id(db: DB, id: i64) -> Result<Self, AppError> {
        let proposal = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    freelancer_id, 
                    cover_letter, 
                    job_id, 
                    bid_amount, 
                    job_type AS "job_type: JobType", 
                    status AS "status: ProposalStatus"
                FROM proposals
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        Ok(proposal)
    }

    pub async fn get_by_job_id(db: DB, id: i64) -> Result<Self, AppError> {
        let proposal = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    freelancer_id, 
                    cover_letter, 
                    job_id, 
                    bid_amount, 
                    job_type AS "job_type: JobType", 
                    status AS "status: ProposalStatus"
                FROM proposals
                WHERE job_id = $1
            "#,
            id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        Ok(proposal)
    }

    pub async fn get_by_freelancer_id(db: DB, id: i64) -> Result<Self, AppError> {
        let proposal = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    freelancer_id, 
                    cover_letter, 
                    job_id, 
                    bid_amount, 
                    job_type AS "job_type: JobType", 
                    status AS "status: ProposalStatus"
                FROM proposals
                WHERE freelancer_id = $1
            "#,
            id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        Ok(proposal)
    }
}
