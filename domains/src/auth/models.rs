use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpAuth {
    pub email: String,
    pub password: String,
    pub confirmation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignInAuth {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
