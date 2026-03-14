use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, ModeratorId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSuspiciousUserUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: ModeratorId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub low_trust_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSuspiciousUserMessage {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub low_trust_status: String,
    pub shared_ban_channel_ids: Option<Vec<UserId>>,
    pub types: Vec<String>,
    pub ban_evasion_evaluation: String,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub text: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
    pub cheermote: Option<Cheermote>,
    pub emote: Option<Emote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheermote {
    pub prefix: String,
    pub bits: u64,
    pub tier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub emote_set_id: String,
}
