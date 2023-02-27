use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use errors::{AppError, ClientError, Errors};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::SetupConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,          // Expiration time (UTC timestamp)
    pub aud: Option<String>, // Audience
    pub iat: usize,          // Issued at (as UTC timestamp)
    pub iss: String,         // Issuer
    pub nbf: usize,          // Not Before (as UTC timestamp)
    pub sub: String,         // Subject (whom token refers to)
}

pub fn decode_claims(token: &str) -> Result<Claims, AppError> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SetupConfig::config.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(c) => Ok(c.claims),
        Err(_) => Err(AppError::new(Errors::Client(ClientError::Unauthorized {
            reason: "Invalid token provided.".into(),
        }))),
    }
}

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}
