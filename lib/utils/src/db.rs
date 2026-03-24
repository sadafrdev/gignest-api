use sqlx::{PgPool, Pool, Postgres};
use dotenvy::dotenv;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub type DB = PgPool;

pub async fn establish_connection() -> Result<Pool<Postgres>, AppError> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    PgPool::connect(&database_url).await.map_err(|_| AppError::InternalServerError)
}
