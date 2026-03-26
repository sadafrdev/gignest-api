use dotenvy;
use serde::{de::DeserializeOwned};
pub mod db;
pub mod enums;
pub mod error;
pub mod middleware;
pub mod jwt;
pub mod encryption;

pub struct ENV;

impl ENV {
    pub fn load<T: DeserializeOwned>() -> T {
        Self::load_file();

        envy::from_env::<T>().expect("Failed to load env")
    }

    pub fn load_file(){
        dotenvy::dotenv().ok();
    }
}