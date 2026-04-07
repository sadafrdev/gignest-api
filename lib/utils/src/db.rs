use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use crate::{ENV, error::AppError};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub type DB = PgPool;

#[derive(Clone, Serialize, Deserialize)]
pub struct Env {
    database_url: String,
}

pub async fn establish_connection() -> Result<Pool<Postgres>, AppError> {
    let env_var: Env = ENV::load();
    let database_url = env_var.database_url;

    PgPool::connect(&database_url)
        .await
        .map_err(|_| AppError::InternalServerError)
}
