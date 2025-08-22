use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    AutoModSetting, AutoModStatus, BanUser, BannedUser, BlockedTerm, ModeratedChannel, Moderator,
    ShieldModeStatus, UnbanRequest, WarnChatUser,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckAutoModStatusResponse {
    pub data: Vec<AutoModStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoModSettingsResponse {
    pub data: Vec<AutoModSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBannedUsersResponse {
    pub data: Vec<BannedUser>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanUsersResponse {
    pub data: Vec<BanUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnbanRequestResponse {
    pub data: Vec<UnbanRequest>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedTermsResponse {
    pub data: Vec<BlockedTerm>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratedChannelResponse {
    pub data: Vec<ModeratedChannel>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratorsResponse {
    pub data: Vec<Moderator>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldModeStatusResponse {
    pub data: Vec<ShieldModeStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarnChatUsersResponse {
    pub data: Vec<WarnChatUser>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::moderation::response::{CheckAutoModStatusResponse, GetBannedUsersResponse};

    #[test]
    fn check_automod_status_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "msg_id": "msg123",
                    "is_permitted": true
                },
                {
                    "msg_id": "msg456",
                    "is_permitted": false
                }
            ]
        });

        let response: CheckAutoModStatusResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_status = &response.data[0];
        assert_eq!(first_status.msg_id, "msg123");
        assert!(first_status.is_permitted);

        let second_status = &response.data[1];
        assert_eq!(second_status.msg_id, "msg456");
        assert!(!second_status.is_permitted);
    }

    #[test]
    fn banned_users_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "banned_user_1",
                    "user_login": "banneduser1",
                    "user_name": "BannedUser1",
                    "expires_at": "2024-01-16T10:30:00Z",
                    "created_at": "2024-01-15T10:30:00Z",
                    "reason": "Spam",
                    "moderator_id": "mod123",
                    "moderator_login": "moderator",
                    "moderator_name": "Moderator"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: GetBannedUsersResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());

        let banned_user = &response.data[0];
        assert_eq!(banned_user.user_id.as_str(), "banned_user_1");
        assert_eq!(banned_user.user_login, "banneduser1");
        assert_eq!(banned_user.reason, "Spam");
        assert_eq!(banned_user.moderator_id.as_str(), "mod123");
    }
}
