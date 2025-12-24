use anyhow::Context;
use dotenvy::dotenv;
use sqlx::PgPool;

pub async fn connect() -> anyhow::Result<PgPool> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    let database_url = &database_url;

    PgPool::connect(database_url)
        .await
        .with_context(|| "failed to connect to Postgres")
}
