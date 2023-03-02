use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use errors::{AppError, ClientError, Errors};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::setup_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,          // Expiration time (UTC timestamp)
    pub aud: Option<String>, // Audience
    pub iat: usize,          // Issued at (as UTC timestamp)
    pub iss: String,         // Issuer
    pub nbf: Option<usize>,  // Not Before (as UTC timestamp)
    pub sub: String,         // Subject (whom token refers to)
}

pub fn decode_claims(token: &str) -> Result<Claims, AppError> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(setup_config::CONFIG.jwt_secret.as_ref()),
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

pub fn verify_password(password: &String, password_received: String) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(password)?;
    Argon2::default().verify_password(password_received.as_bytes(), &parsed_hash)?;
    Ok(())
}

pub fn encode_token(entity_id: String) -> Result<String, AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: Claims = Claims {
        iat,
        iss: env::var("WEB_SERVER").map_or_else(|_| "wsstudy".to_string(), |v| v),
        sub: entity_id,
        exp,
        aud: Some(format!("{}/api/", setup_config::CONFIG.format_server_url())),
        nbf: None,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(setup_config::CONFIG.jwt_secret.as_ref()),
    )?;

    Ok(token)
}
