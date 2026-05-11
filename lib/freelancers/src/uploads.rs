use axum::{extract::Multipart, response::IntoResponse};
use std::path::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Default, Serialize)]
pub struct Uploads{
    id: i64,
    user_id: i64,
    title: String,
    description: String,
    original_name: String,
    safe_name: String,
    file_path: String,
    file_bytes: Vec<u8>,
}

impl Uploads {
    pub async fn get_uploads(db: DB, user_id: i64) -> Result<Vec<Self>, AppError> {
        let uploads = sqlx::query_as!(
            Self,
            "
                SELECT 
                    id,
                    user_id,
                    title,
                    description,
                    original_name,
                    safe_name,
                    file_path,
                    file_bytes
                FROM uploads 
                WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(uploads)
    }

    pub async fn download(
        id: i64,
        db: DB,
    ) -> Result<impl IntoResponse, AppError> {
        let upload = sqlx::query_as!(
            Self,
            "
                SELECT 
                    id,
                    user_id,
                    title,
                    description,
                    original_name,
                    safe_name,
                    file_path,
                    file_bytes
                FROM uploads 
                WHERE id = $1
            ",
            id
        )
        .fetch_one(&db)
        .await
        .map_err(|_| AppError::BadRequest("Upload not found"))?;

        let file = tokio::fs::read(&upload.file_path)
            .await
            .map_err(|_| AppError::BadRequest("File not found on disk"))?;
    
        Ok((
            [("Content-Disposition", format!("attachment; filename=\"{}\"", upload.original_name))],
            file,
        ))
    }

    pub async fn create(
        mut multi: Multipart,
        db : DB
    )-> Result<(), AppError> {
        let mut form = Self::default();
        while let Some(field) = multi.next_field().await.map_err(|_| AppError::BadRequest("Failed to read multipart field"))? {
            let name = field.name().unwrap_or_default().to_string();

            if field.file_name().is_some() {
                form.original_name = field.file_name().unwrap().to_string();

                let ext = Path::new(&form.original_name)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("bin");

                form.safe_name = format!("{}.{}", Uuid::new_v4(), ext);
                form.file_bytes = field.bytes().await.map_err(|_| AppError::BadRequest("Failed to read file size"))?.to_vec();
            } else {
                let value = field.text().await.map_err(|_| AppError::BadRequest("Failed to read form field value"))?;
                match name.as_str(){
                    "title" => form.title = value,
                    "description" => form.description = value,
                    "user_id" => form.user_id = value.parse().unwrap(),
                    _ => {}
                }
            }
        }

        let file_path = format!("/tmp/{}", form.safe_name);

        tokio::fs::create_dir_all("uploads")
        .await
        .map_err(|_| AppError::BadRequest("Failed to create upload dir"))?;

        tokio::fs::write(&file_path, &form.file_bytes).await.map_err(|_| AppError::BadRequest("Failed to write file bytes"))?;

        sqlx::query!(
            "
                INSERT INTO uploads (user_id, title, description, original_name, safe_name, file_path, file_bytes) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
            form.user_id,
            form.title,
            form.description,
            form.original_name,
            form.safe_name,
            file_path,
            form.file_bytes
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn delete(
        id: i64,
        db: DB
    ) -> Result<(), AppError> {
        let upload = sqlx::query!(
            "
                DELETE FROM uploads 
                WHERE id = $1 
                RETURNING file_path
            ",
            id
        )
        .fetch_one(&db)
        .await?;
    
        tokio::fs::remove_file(&upload.file_path).await.map_err(|_| AppError::BadRequest("Failed to delete file from disk"))?;
    
        Ok(())
    }
    
}
