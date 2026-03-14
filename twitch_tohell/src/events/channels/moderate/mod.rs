pub mod v1;
pub mod v2;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowersModeration {
    pub follow_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowModeration {
    pub wait_time_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnvipModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmodModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub reason: Option<String>,
    pub expires_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntimeoutModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub viewer_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnraidModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub message_id: String,
    pub message_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodTermsModeration {
    pub action: String,
    pub list: String,
    pub terms: Vec<String>,
    pub from_automod: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanRequestModeration {
    pub is_approved: bool,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub moderator_message: String,
}
