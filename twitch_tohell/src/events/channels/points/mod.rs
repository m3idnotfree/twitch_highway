pub mod v1;
pub mod v2;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, Images, RedemptionId, Reward, RewardId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsCustomRewardAdd {
    pub id: RewardId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub is_enabled: bool,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub title: String,
    pub cost: u64,
    pub prompt: String,
    pub is_user_input_required: bool,
    pub should_redemptions_skip_request_queue: bool,
    pub max_per_stream: Option<MaxPerStream>,
    pub max_per_user_per_stream: Option<MaxPerUserPerStream>,
    pub background_color: String,
    pub image: Option<Images>,
    pub default_image: Images,
    pub global_cooldown: Option<GlobalCooldown>,
    pub cooldown_expires_at: Option<DateTime<FixedOffset>>,
    pub redemptions_redeemed_current_stream: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsCustomRewardUpdate {
    pub id: RewardId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub is_enabled: bool,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub title: String,
    pub cost: u64,
    pub prompt: String,
    pub is_user_input_required: bool,
    pub should_redemptions_skip_request_queue: bool,
    pub max_per_stream: Option<MaxPerStream>,
    pub max_per_user_per_stream: Option<MaxPerUserPerStream>,
    pub background_color: String,
    pub image: Option<Images>,
    pub default_image: Images,
    pub global_cooldown: Option<GlobalCooldown>,
    pub cooldown_expires_at: Option<DateTime<FixedOffset>>,
    pub redemptions_redeemed_current_stream: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsCustomRewardRemove {
    pub id: RewardId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub is_enabled: bool,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub title: String,
    pub cost: u64,
    pub prompt: String,
    pub is_user_input_required: bool,
    pub should_redemptions_skip_request_queue: bool,
    pub max_per_stream: Option<MaxPerStream>,
    pub max_per_user_per_stream: Option<MaxPerUserPerStream>,
    pub background_color: String,
    pub image: Option<Images>,
    pub default_image: Images,
    pub global_cooldown: Option<GlobalCooldown>,
    pub cooldown_expires_at: Option<DateTime<FixedOffset>>,
    pub redemptions_redeemed_current_stream: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsCustomRewardRedemptionAdd {
    pub id: RedemptionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub user_input: String,
    pub status: String,
    pub reward: Reward,
    pub redeemed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsCustomRewardRedemptionUpdate {
    pub id: RedemptionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub user_input: String,
    pub status: String,
    pub reward: Reward,
    pub redeemed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxPerStream {
    pub is_enabled: bool,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxPerUserPerStream {
    pub is_enabled: bool,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCooldown {
    pub is_enabled: bool,
    pub seconds: u32,
}
