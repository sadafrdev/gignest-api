use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::chrono::NaiveDate};
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Certificate {
    pub user_id: Option<i64>,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

impl Certificate {
    pub async fn generate(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            " INSERT INTO certificates (user_id, name, certificate_by, year) VALUES ($1, $2, $3, $4)",
            self.user_id,
            self.name,
            self.certificate_by,
            self.year,
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct UpdateCertificate {
    pub id: i64,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

impl UpdateCertificate{
    pub async fn update(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE certificates
                SET
                    name = $1,
                    certificate_by = $2,
                    year = $3
                WHERE id = $4
            ",
            self.name,
            self.certificate_by,
            self.year,
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CertificateDelete {
   id: i64
}

impl CertificateDelete {
    pub async fn delete(
        self, db: PgPool
    ) -> Result<(), AppError> {
        sqlx::query!(
            " DELETE FROM certificates WHERE id = $1",
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: Option<i64>
}

impl  User{
    pub async fn get(
        self,
        db: DB
    ) -> Result<Vec<Certificate>, AppError> {
        let certificates = sqlx::query_as!(
            Certificate,
            "
                SELECT
                    user_id,
                    name,
                    certificate_by,
                    year
                FROM certificates
                WHERE user_id = $1
            ",
            self.user_id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;

        Ok(certificates)
    }
}