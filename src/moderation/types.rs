use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    serde_helpers::deserialize_optional_datetime,
    types::{BlockedTermId, BroadcasterId, ModeratorId, UnbanRequestId, UserId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoModStatus {
    pub msg_id: String,
    pub is_permitted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoModSetting {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannedUser {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    #[serde(default, deserialize_with = "deserialize_optional_datetime")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    pub created_at: DateTime<FixedOffset>,
    pub reason: String,
    pub moderator_id: ModeratorId,
    pub moderator_login: String,
    pub moderator_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanUser {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub user_id: UserId,
    pub created_at: DateTime<FixedOffset>,
    pub end_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanRequest {
    pub id: UnbanRequestId,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTerm {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub id: BlockedTermId,
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratedChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moderator {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldModeStatus {
    pub is_active: bool,
    pub moderator_id: ModeratorId,
    pub moderator_name: String,
    pub moderator_login: String,
    pub last_activated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarnChatUser {
    pub broadcaster_id: BroadcasterId,
    pub user_id: UserId,
    pub moderator_id: ModeratorId,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckAutoMod {
    msg_id: String,
    msg_text: String,
}

impl CheckAutoMod {
    pub fn new(msg_id: impl Into<String>, msg_text: impl Into<String>) -> Self {
        Self {
            msg_id: msg_id.into(),
            msg_text: msg_text.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutoModAction {
    ALLOW,
    DENY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SuspiciousStatus {
    ActiveMonitoring,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SuspiciousType {
    ManuallyAdded,
    DetectedBanEvader,
    DetectedSusChatter,
    BannedInSharedChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousUser {
    pub user_id: UserId,
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub updated_at: String,
    pub status: SuspiciousStatus,
    pub types: Vec<SuspiciousType>,
}

#[derive(Serialize)]
pub struct CheckAutomodStatusBody<'a> {
    pub data: &'a [CheckAutoMod],
}

#[derive(Serialize)]
pub struct ManageHeldAutomodMessagesBody<'a> {
    pub user_id: &'a UserId,
    pub msg_id: &'a str,
    pub action: AutoModAction,
}

#[derive(Serialize)]
pub struct AddBlockedTermBody<'a> {
    pub text: &'a str,
}

#[derive(Serialize)]
pub struct UpdateShieldModeStatusBody {
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct WarnChatUserBodyWrapper<'a> {
    pub data: WarnChatUserBody<'a>,
}

#[derive(Serialize)]
pub struct WarnChatUserBody<'a> {
    pub user_id: &'a UserId,
    pub reason: &'a str,
}

#[derive(Serialize)]
pub struct SuspiciousBody<'a> {
    pub user_id: &'a UserId,
    pub status: SuspiciousStatus,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::moderation::{BannedUser, UnbanRequestStatus};

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

    #[test]
    fn banned_user() {
        let input = json!({
              "user_id": "78325877",
              "user_login": "fisherskateboard566",
              "user_name": "FisherSkateboard566",
              "expires_at": "",
              "created_at": "2024-12-22T04:49:22Z",
              "reason": "CLI ban",
              "moderator_id": "6549720",
              "moderator_login": "fishersteve14",
              "moderator_name": "FisherSteve14"
        });

        serde_json::from_value::<BannedUser>(input).unwrap();
    }
}
