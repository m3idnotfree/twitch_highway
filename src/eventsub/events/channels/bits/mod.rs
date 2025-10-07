use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelBitsUse {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub bits: String,
    #[serde(rename = "type")]
    pub kind: BitsUseType,
    pub message: Option<Message>,
    pub power_up: Option<PowerUp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BitsUseType {
    Cheer,
    PowerUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    pub text: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub emote: Option<Emote>,
    pub cheermote: Option<Cheermote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub emote_set_id: String,
    pub owner_id: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheermote {
    pub prefix: String,
    pub bits: u64,
    pub tier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerUp {
    #[serde(rename = "type")]
    pub kind: PowerType,
    pub emote: Option<PowerEmote>,
    pub message_effect_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerType {
    MessageEffect,
    Celebration,
    GiganifyAnEmote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerEmote {
    pub id: String,
    pub name: String,
}
