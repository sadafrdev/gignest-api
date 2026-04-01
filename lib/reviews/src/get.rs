use utils::{db::DB, error::AppError};
use crate::create::Review;

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