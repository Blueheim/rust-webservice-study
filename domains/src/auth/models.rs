use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpAuth {
    pub email: String,
    pub password: String,
    pub confirmation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInAuth {
    pub email: String,
    pub password: String,
}
