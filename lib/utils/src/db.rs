use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub type DB = PgPool;

pub async fn establish_connection() -> Result<AppState> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    let db = PgPool::connect(&database_url).await?;

    Ok(AppState { db })
}
