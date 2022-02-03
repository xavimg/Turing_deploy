use bson::{doc};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerToken {
    Loged(PlayerTokenLoged),
    Unloged(u64)
}

impl PlayerToken {
    pub const fn get_id (&self) -> u64 {
        match self {
            Self::Loged(x) => x.id,
            Self::Unloged(x) => *x
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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