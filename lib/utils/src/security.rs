use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::PasswordHash;
use crate::error::AppError;
use argon2::{
    password_hash::{SaltString, PasswordHasher, rand_core::OsRng}
};

pub fn verify(db_password: &str, password: String ) -> Result<(), AppError>{
    let parsed_hash = PasswordHash::new(db_password).map_err(|_| AppError::InternalServerError)?;

    Argon2::default()    
    .verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|_| AppError::ValidationError("Invalid credentials"))?;

    Ok(())
}

pub fn hash_password(pswd: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(pswd.as_bytes(), &salt)?
        .to_string();

    Ok(hashed_password)
}