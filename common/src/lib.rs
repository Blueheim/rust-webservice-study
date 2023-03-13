use serde::{Deserialize, Serialize};

pub mod crypto;
pub mod validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessPayload<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload<T> {
    pub errors: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoPayload {
    pub message: String,
}

pub const CAT_SCOPE: &str = "/cats";
