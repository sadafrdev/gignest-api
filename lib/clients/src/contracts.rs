use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Serialize, Debug)]
pub struct AcceptProposal {
    client_id: i64,
    freelancer_id: i64,
    job_id: i64,
    status: ContractStatus
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "contract_status")]
pub enum ContractStatus {
    Active,
    Completed
}

impl AcceptProposal {
    pub async fn accept_proposal_and_create_contract(self, db: DB) -> Result<(), AppError> {
       let record = sqlx::query!(
            r#"
                SELECT P.id
                FROM proposals P
                JOIN jobs J ON P.job_id = J.id
                WHERE P.job_id = $1
                AND J.client_id = $2
                AND P.freelancer_id = $3
            "#,
            self.job_id,
            self.client_id,
            self.freelancer_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Unauthorized)?;  

       sqlx::query!(
            "UPDATE proposals SET status = 'Accepted' WHERE id = $1 ",
            record.id
        )
        .execute(&db)
        .await?;

        sqlx::query!(
            "UPDATE proposals SET status = 'Rejected' WHERE job_id = $1 AND id != $2",
            self.job_id,
            record.id
        )
        .execute(&db)
        .await?;

        sqlx::query!(
            " INSERT INTO contract (client_id, freelancer_id, job_id, status) VALUES ($1, $2, $3, $4)",
            self.client_id,
            self.freelancer_id,
            self.job_id,
            self.status as ContractStatus
        )
        .execute(&db)
        .await
        .inspect_err(|e| eprintln!("SQL ERROR: {e:?}"))
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }

}

pub async fn delete_contract(id: i64, db: DB) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM contract WHERE id = $1",
        id
    )
    .execute(&db)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(())
}