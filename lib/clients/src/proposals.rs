use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{enums::{JobType, ProposalStatus}, error::AppError, db::DB};
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    user_id: i64,
    cover_letter: String,
    job_id: i64,
    bid_amount: BigDecimal,
    job_type: JobType,
    status: ProposalStatus,
}

impl Proposal {
    pub async fn create(self, db: DB) -> Result<(), AppError>{
        sqlx::query!(
            "
                INSERT INTO proposals (user_id, cover_letter, job_id, bid_amount, job_type, status)
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
            self.user_id,
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