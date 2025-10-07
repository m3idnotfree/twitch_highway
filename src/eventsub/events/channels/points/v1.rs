use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, RedemptionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsAutomaticRewardRedemptionAdd {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub id: RedemptionId,
    pub reward: Reward,
    pub message: Message,
    pub user_input: Option<String>,
    pub redeemed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    #[serde(rename = "type")]
    pub kind: RewardType,
    pub cost: u64,
    pub unlocked_emote: Option<UnlockedEmote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardType {
    SingleMessageBypassSubMode,
    SendHighlightedMessage,
    RandomSubEmoteUnlock,
    ChosenSubEmoteUnlock,
    ShosenModifiedSubEmoteUnlock,
    MessageEffect,
    GigantifyAnEmote,
    Celebration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockedEmote {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub emotes: Vec<Emote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub begin: u64,
    pub end: u64,
}
