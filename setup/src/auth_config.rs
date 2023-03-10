use std::env;

use chrono::{Duration, Utc};
use errors::{AppError, ClientError, Errors};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::setup_config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    /// Jwt token secret
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,          // Expiration time (UTC timestamp)
    pub aud: Option<String>, // Audience
    pub iat: usize,          // Issued at (as UTC timestamp)
    pub iss: String,         // Issuer
    pub nbf: Option<usize>,  // Not Before (as UTC timestamp)
    pub sub: String,         // Subject (whom token refers to)
}

impl AuthConfig {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

        Self { jwt_secret }
    }

    pub fn decode_claims(&self, token: &str) -> Result<Claims, AppError> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => Ok(c.claims),
            Err(_) => Err(AppError::new(Errors::Client(ClientError::Unauthorized {
                reason: "Invalid token provided.".into(),
            }))),
        }
    }

    pub fn encode_token(&self, entity_id: String) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(60)).timestamp() as usize;
        let claims: Claims = Claims {
            iat,
            iss: env::var("WEB_SERVER").map_or_else(|_| "wsstudy".to_string(), |v| v),
            sub: entity_id,
            exp,
            aud: Some(format!(
                "{}/api/",
                setup_config::APP_CONFIG.server.format_url()
            )),
            nbf: None,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }
}
