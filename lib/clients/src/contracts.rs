use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Serialize, Debug)]
pub struct Contract {
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

impl Contract {
    pub async fn accept_proposal_and_create_contract(self, db: DB) -> Result<(), AppError> {
       let record = sqlx::query!(
            "
                SELECT P.id
                FROM proposals P
                JOIN jobs J ON P.job_id = J.id
                WHERE P.job_id = $1
                AND J.client_id = $2
                AND P.freelancer_id = $3
            ",
            self.job_id,
            self.client_id,
            self.freelancer_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Unauthorized)?;  

        sqlx::query!(
            r#"
                WITH accept_proposal AS (
                    UPDATE proposals SET status = 'Accepted' WHERE id = $1
                ),
                reject_other_proposals AS (
                   UPDATE proposals SET status = 'Rejected' WHERE job_id = $2 AND id != $1
                )
                INSERT INTO contract (client_id, freelancer_id, job_id, status) 
                VALUES ($3, $4, $5, $6)
            "#,
            record.id,
            self.job_id,
            self.client_id,
            self.freelancer_id,
            self.job_id,
            self.status as ContractStatus
        )
        .execute(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
    
    pub async fn freelancer_contracts_by_id(id: i64, db: DB) -> Result<Vec<Self>, AppError> {
        let contracts = sqlx::query_as!(
            Self,
            r#"
                SELECT 
                    client_id, 
                    freelancer_id, 
                    job_id,
                    status AS "status: ContractStatus" 
                FROM contract 
                WHERE freelancer_id = $1
            "#,
            id
        )
        .fetch_all(&db)
        .await?;

        Ok(contracts)
    }

    pub async fn client_contracts_by_id(id: i64, db: DB) -> Result<Vec<Self>, AppError> {
        let contracts = sqlx::query_as!(
            Self,
            r#"
                SELECT 
                    client_id, 
                    freelancer_id, 
                    job_id, 
                    status AS "status: ContractStatus" 
                FROM contract 
                WHERE client_id = $1
            "#,
            id
        )
        .fetch_all(&db)
        .await?;
    
        Ok(contracts)
    }
    
}

pub async fn complete_contract(id: i64, db: DB) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE contract SET status = 'Completed' WHERE id = $1",
        id
    )
    .execute(&db)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(())
}
