use common::PASSWORD_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignUpAuth {
    #[validate(email)]
    pub email: String,
    #[validate(regex = "PASSWORD_REGEX")]
    pub password: String,
    #[validate(must_match = "password")]
    pub confirmation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignInAuth {
    #[validate(email)]
    pub email: String,
    #[validate(regex = "PASSWORD_REGEX")]
    pub password: String,
}
