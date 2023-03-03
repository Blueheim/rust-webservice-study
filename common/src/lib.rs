use serde::{Deserialize, Serialize};
use validator::ValidationError;

pub mod validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessPayload<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload<T> {
    pub error: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoPayload {
    pub message: String,
}
