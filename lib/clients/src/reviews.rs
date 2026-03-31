use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Review {
    pub id: i64,
    pub reviewer_id: i64,
    pub reviewee_id: i64,
    pub contract_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
}

impl Review {
    pub async fn create_review(
        self,
        db: DB,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO reviews (contract_id, reviewer_id, reviewee_id, rating, comment) VALUES ($1, $2, $3, $4, $5)",
            self.contract_id,
            self.reviewer_id,
            self.reviewee_id,
            self.rating,
            self.comment
        )
        .execute(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}

pub async fn get_reviews_by_reviewee_id(
    reviewee_id: i64,
    db: DB,
) -> Result<Vec<Review>, AppError> {
    let reviews = sqlx::query_as!(
        Review,
        "
            SELECT 
                id, 
                reviewer_id, 
                reviewee_id, 
                contract_id, 
                rating, 
                comment 
            FROM reviews 
            WHERE reviewee_id = $1",
        reviewee_id
    )
    .fetch_optional(&db)
    .await;

    Ok(reviews)
}