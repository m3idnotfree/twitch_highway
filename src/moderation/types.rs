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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub id: Id,
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
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
pub struct WarnChatUser {
    pub broadcaster_id: BroadcasterId,
    pub user_id: UserId,
    pub moderator_id: ModeratorId,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use crate::moderation::types::UnbanRequestStatus;

    #[test]
    fn unban_request_status_enum() {
        let statuses = vec![
            (UnbanRequestStatus::Pending, "pending"),
            (UnbanRequestStatus::Approved, "approved"),
            (UnbanRequestStatus::Denied, "denied"),
            (UnbanRequestStatus::Acknowledged, "acknowledged"),
            (UnbanRequestStatus::Canceled, "canceled"),
        ];

        for (status, expected_str) in statuses {
            assert_eq!(status.as_str(), expected_str);
            assert_eq!(status.as_ref(), expected_str);

            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: UnbanRequestStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }
}
