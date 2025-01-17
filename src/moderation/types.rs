use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, ModeratorId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoModStatus {
    pub msg_id: String,
    pub is_permitted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoModSetting {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: String,
    pub overall_level: Option<u64>,
    pub disability: u64,
    pub aggression: u64,
    pub sexuality_sex_or_gender: u64,
    pub misogyny: u64,
    pub bullying: u64,
    pub swearing: u64,
    pub race_ethnicity_or_religion: u64,
    pub sex_based_terms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannedUser {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub expires_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub reason: String,
    pub moderator_id: ModeratorId,
    pub moderator_login: String,
    pub moderator_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanUser {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub user_id: UserId,
    pub created_at: DateTime<FixedOffset>,
    pub end_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnbanRequestStatus {
    Pending,
    Approved,
    Denied,
    Acknowledged,
    Canceled,
}

impl UnbanRequestStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Acknowledged => "acknowledged",
            Self::Canceled => "canceled",
        }
    }
}

impl AsRef<str> for UnbanRequestStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnbanRequest {
    pub id: Id,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub moderator_login: String,
    pub moderator_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub text: String,
    pub status: UnbanRequestStatus,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub resolution_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedTerm {
    broadcaster_id: BroadcasterId,
    moderator_id: ModeratorId,
    id: Id,
    text: String,
    created_at: String,
    updated_at: String,
    expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratedChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Moderator {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldModeStatus {
    pub is_active: bool,
    pub moderator_id: ModeratorId,
    pub moderator_name: String,
    pub moderator_login: String,
    pub last_activated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarnChatUserff {
    pub broadcaster_id: BroadcasterId,
    pub user_id: UserId,
    pub moderator_id: ModeratorId,
    pub reason: String,
}
