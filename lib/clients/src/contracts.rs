use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, error::AppError};
use axum::Json;

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

pub async fn freelancer_contracts(id: i64, db: DB) -> Result<Json<Vec<Contract>>, AppError> {
    let contracts = sqlx::query_as!(
        Contract,
        r#"SELECT client_id, freelancer_id, job_id, status as "status: ContractStatus" FROM contract WHERE freelancer_id = $1"#,
        id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(contracts))
}

pub async fn client_contracts(id: i64, db: DB) -> Result<Json<Vec<Contract>>, AppError> {
    let contracts = sqlx::query_as!(
        Contract,
        r#"SELECT client_id, freelancer_id, job_id, status as "status: ContractStatus" FROM contract WHERE client_id = $1"#,
        id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(contracts))
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
