use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Review {
    pub reviewer_id: Option<i64>,
    pub reviewee_id: Option<i64>,
    pub contract_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
}

impl Review {
    pub async fn create(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                INSERT INTO reviews (contract_id, reviewer_id, reviewee_id, rating, comment) 
                VALUES ($1, $2, $3, $4, $5)
            ",
            self.contract_id,
            self.reviewer_id,
            self.reviewee_id,
            self.rating,
            self.comment
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn find_reviews_by_reviewee_id(
        reviewee_id: i64,
        db: DB,
    ) -> Result<Vec<Self>, AppError> {
        let reviews = sqlx::query_as!(
            Self,
            "
                SELECT 
                    reviewer_id, 
                    reviewee_id, 
                    contract_id,
                    rating, 
                    comment 
                FROM reviews 
                WHERE reviewee_id = $1",
            reviewee_id
        )
        .fetch_all(&db)
        .await?;

        Ok(reviews)
    }
}
