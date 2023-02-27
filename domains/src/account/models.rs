use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountId(pub uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    pub creation_time: Option<DateTime<Utc>>,
    pub last_modification_time: Option<DateTime<Utc>>,
}

impl Account {
    pub fn mock_data() -> Vec<Self> {
        let file = include_str!("./mock/accounts.json");
        serde_json::from_str(file).expect("can't read accounts.json")
    }
}
