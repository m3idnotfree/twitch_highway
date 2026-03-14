use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, EmoteId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSubscriptionEnd {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub tier: String,
    pub is_gift: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSubscriptionGift {
    pub user_id: Option<UserId>,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub total: u32,
    pub tier: String,
    pub cumulative_total: Option<u32>,
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSubscriptionMessage {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub tier: String,
    pub message: Message,
    pub cumulative_months: u32,
    pub streak_months: Option<u32>,
    pub duration_months: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub emotes: Vec<Emote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    begin: u64,
    end: u64,
    id: EmoteId,
}
