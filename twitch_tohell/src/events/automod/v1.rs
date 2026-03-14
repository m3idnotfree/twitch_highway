use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    UserId,
    events::automod::{Cheermote, Status},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodMessageHold {
    pub broadcaster_user_id: UserId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub message_id: String,
    pub message: Message,
    pub category: String,
    pub level: u32,
    pub held_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodMessageUpdate {
    pub broadcaster_user_id: UserId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub moderator_user_id: UserId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub message_id: String,
    pub message: Message,
    pub category: String,
    pub level: u32,
    pub status: Status,
    pub held_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    pub text: String,
    pub emote: Option<Emote>,
    pub cheermote: Option<Cheermote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub emote_set_id: String,
}
