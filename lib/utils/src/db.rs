use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn establish_connection() -> Result<AppState, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    let database_url = &database_url;
    let db = PgPool::connect(&database_url).await?;

    let state = AppState { db };

    Ok(state)
}
