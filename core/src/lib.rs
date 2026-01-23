pub mod utils;
use sqlx::{Pool, Postgres};
pub use authentication;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}
