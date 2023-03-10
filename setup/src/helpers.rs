use std::collections::HashMap;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use config::Config;
use errors::AppError;

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}

pub fn verify_password(password: &str, password_received: String) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(password)?;
    Argon2::default().verify_password(password_received.as_bytes(), &parsed_hash)?;
    Ok(())
}

pub fn read_config_file() -> HashMap<String, String> {
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/config");
    let config = Config::builder()
        .add_source(config::File::with_name(file))
        .build()
        .unwrap();

    config.try_deserialize::<HashMap<String, String>>().unwrap()
}
