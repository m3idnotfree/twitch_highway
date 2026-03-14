use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, RedemptionId, UserId};

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
    pub message: Option<Message>,
    pub redeemed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    #[serde(rename = "type")]
    pub kind: RewardType,
    pub channel_points: u64,
    pub emote: Option<RewardEmote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEmote {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewardType {
    SingleMessageBypassSubMode,
    SendHighlightedMessage,
    RandomSubEmoteUnlock,
    ChosenSubEmoteUnlock,
    ChosenModifiedSubEmoteUnlock,
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
    pub kind: FragmenetType,
    pub emote: Emote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FragmenetType {
    Text,
    Emote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
}
