pub mod utils;
use sqlx::{Pool, Postgres};
pub mod clients;
pub mod authentication;
pub mod freelancers;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}
