use bson::{doc};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PlayerToken {
    Loged(String),
    Unloged(u64)
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PlayerTokenLoged {
    #[serde(rename = "user_id")]
    pub id: u64,
    #[serde(rename = "exp", with = "ts_seconds")]
    pub expiration_date: DateTime<Utc>,
    #[serde(rename = "iat", with = "ts_seconds")]
    pub issued_at: DateTime<Utc>,
    #[serde(rename = "iss")]
    pub issued_by: String
}

impl PlayerTokenLoged {
    pub fn default_for (id: u64) -> Self {
        let now = Utc::now();
        Self {
            id,
            expiration_date: now + Duration::days(3),
            issued_at: now,
            issued_by: "Testing".to_string()
        }
    }
}