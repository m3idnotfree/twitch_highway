use std::fmt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Images, RedemptionId, Reward, RewardId, UserId};

#[derive(Debug, Clone, Deserialize)]
pub struct CustomReward {
    pub broadcaster_id: BroadcasterId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_login: Option<String>,
    pub broadcaster_name: String,
    pub id: RewardId,
    pub title: String,
    pub prompt: String,
    pub cost: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Images>,
    pub default_image: Images,
    pub background_color: String,
    pub is_enabled: bool,
    pub is_user_input_required: bool,
    pub max_per_stream_setting: MaxPerStreamSetting,
    pub max_per_user_per_stream_setting: MaxPerStreamSetting,
    pub global_cooldown_setting: MaxPerStreamSetting,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub should_redemptions_skip_request_queue: bool,
    pub redemptions_redeemed_current_stream: Option<u64>,
    pub cooldown_expires_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CustomRewardsRedemption {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub id: RedemptionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
    pub user_id: UserId,
    pub user_name: String,
    pub user_input: String,
    pub status: RedemptionStatus,
    pub redeemed_at: DateTime<FixedOffset>,
    pub reward: Reward,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaxPerStreamSetting {
    pub is_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_user_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cooldown_seconds: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RedemptionStatus {
    CANCELED,
    FULFILLED,
    UNFULFILLED,
}

impl RedemptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CANCELED => "CANCELED",
            Self::FULFILLED => "FULFILLED",
            Self::UNFULFILLED => "UNFULFILLED",
        }
    }
}

impl fmt::Display for RedemptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for RedemptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sort {
    OLDEST,
    NEWEST,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        match self {
            Self::OLDEST => "OLDEST",
            Self::NEWEST => "NEWEST",
        }
    }
}

#[derive(Serialize)]
pub(crate) struct UpdateRedemptionStatusBody {
    pub status: RedemptionStatus,
}
