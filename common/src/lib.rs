#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde::{Deserialize, Serialize};

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

lazy_static! {
    pub static ref PASSWORD_REGEX: Regex = Regex::new(r"[a-z]{2}$").unwrap();
}
