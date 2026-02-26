use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{enums::{JobType, ProposalStatus}, error::AppError, db::DB};
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    freelancer_id: i64,
    cover_letter: String,
    job_id: i64,
    bid_amount: BigDecimal,
    job_type: JobType,
    status: ProposalStatus,
}

impl Proposal {
    pub async fn create(self, db: DB) -> Result<(), AppError>{
        let exists= sqlx::query!(
            r#"
                SELECT job_id
                FROM proposals
                WHERE job_id = $1 AND freelancer_id = $2
            "#,
            self.job_id,
            self.freelancer_id
        )
        .fetch_optional(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        if exists.is_some(){
            println!("Your Proposal for this job Exist");
            return Err(AppError::NotFound("FREELANCER".to_string()));
        }
        
        sqlx::query!(
            "
                INSERT INTO proposals (freelancer_id, cover_letter, job_id, bid_amount, job_type, status)
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
            self.freelancer_id,
            self.cover_letter,
            self.job_id,
            self.bid_amount,
            self.job_type as JobType,
            self.status as ProposalStatus
           
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProposalID {
    id: i64
}

impl  ProposalID{
    pub async fn get_by_proposal_id(
        self, db: DB
    ) -> Result<Option<Proposal>, AppError> {
        let proposal= sqlx::query_as::<_, Proposal>(
            r#"
                SELECT
                    freelancer_id, cover_letter, job_id, bid_amount, job_type, status
                FROM proposals
                WHERE id = $1
            "#,
        )
        .bind(self.id)
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;
        
        Ok(proposal)
    }

    pub async fn get_by_job_id(
        self, db: DB
    ) -> Result<Option<Proposal>, AppError> {
        let proposal= sqlx::query_as::<_, Proposal>(
            r#"
                SELECT
                    freelancer_id, cover_letter, job_id, bid_amount, job_type, status
                FROM proposals
                WHERE job_id = $1
            "#,
        )
        .bind(self.id)
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;
        
        Ok(proposal)
    }

    pub async fn get_by_freelancer_id(
        self, db: DB
    ) -> Result<Option<Proposal>, AppError> {        
        let proposal= sqlx::query_as::<_, Proposal>(
            r#"
                SELECT
                    freelancer_id, cover_letter, job_id, bid_amount, job_type, status
                FROM proposals
                WHERE freelancer_id = $1
            "#,
        )
        .bind(self.id)
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;
        
        Ok(proposal)
    }

}