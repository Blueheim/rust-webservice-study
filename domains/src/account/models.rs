use std::str::FromStr;

use chrono::{DateTime, Utc};
use errors::{AppError, ClientError, Errors};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountId(pub Uuid);

impl FromStr for AccountId {
    type Err = AppError;

    fn from_str(id: &str) -> Result<Self, AppError> {
        match id.is_empty() {
            false => Ok(AccountId(Uuid::try_parse(id)?)),
            true => Err(AppError::new(Errors::Client(ClientError::InvalidId))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    pub creation_time: DateTime<Utc>,
    pub last_modification_time: Option<DateTime<Utc>>,
}

impl Account {
    pub fn mock_data() -> Vec<Self> {
        let file = include_str!("./mock/accounts.json");
        serde_json::from_str(file).expect("can't read accounts.json")
    }
}
