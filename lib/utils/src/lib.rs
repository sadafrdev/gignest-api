use dotenvy;
use serde::de::DeserializeOwned;
pub mod db;
pub mod encryption;
pub mod enums;
pub mod error;
pub mod jwt;
pub mod middleware;

pub struct ENV;

impl ENV {
    pub fn load<T: DeserializeOwned>() -> T {
        Self::load_file();

        envy::from_env::<T>().expect("Failed to load env")
    }

    pub fn load_file() {
        dotenvy::dotenv().ok();
    }
}
