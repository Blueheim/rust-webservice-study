use common::validation::validate_password;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignUpAuth {
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[validate(must_match = "password")]
    pub confirmation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignInAuth {
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}
