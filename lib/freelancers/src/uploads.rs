use axum::extract::{Multipart};
use std::{path::Path};
use serde::{Deserialize};
use uuid::Uuid;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Default)]
pub struct Upload{
    user_id: i64,
    title: String,
    description: String,
    original_name: String,
    safe_name: String,
    file_bytes: Vec<u8>,
}

pub async fn create(
    mut multi: Multipart,
    db : DB
)-> Result<(), AppError> {
    let mut form = Upload::default();

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
                "user_id" => form.user_id = value.parse().unwrap_or(0),
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
        "INSERT INTO uploads (user_id, title, description, original_name, safe_name, file_path) VALUES ($1, $2, $3, $4, $5, $6)",
        form.user_id,
        form.title,
        form.description,
        form.original_name,
        form.safe_name,
        file_path,
    )
    .execute(&db)
    .await?;

    Ok(())
}
