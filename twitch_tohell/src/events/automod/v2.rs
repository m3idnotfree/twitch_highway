use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    UserId,
    events::automod::{Cheermote, Status},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodMessageHoldV2 {
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
    pub reason: Reason,
    pub automod: Option<Automod>,
    pub blocked_term: Option<BlockedTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodMessageUpdateV2 {
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
    pub reason: Reason,
    pub automod: Option<Automod>,
    pub blocked_term: Option<BlockedTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
    pub emote: Option<Emote>,
    pub cheermote: Option<Cheermote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub emote_set_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FragmentType {
    Text,
    Emote,
    Cheermote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
    Automod,
    #[serde(rename = "blocked_term")]
    BlockedTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automod {
    pub category: String,
    pub level: u64,
    pub boundaries: Vec<Boundary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTerm {
    pub terms_found: Vec<TermFound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermFound {
    pub term_id: String,
    pub boundary: Boundary,
    pub owner_broadcaster_user_id: String,
    pub owner_broadcaster_user_login: String,
    pub owner_broadcaster_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boundary {
    pub start_pos: u64,
    pub end_pos: u64,
}
