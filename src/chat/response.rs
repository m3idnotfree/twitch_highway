use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    Badge, ChatSetting, Chatter, Emote, MessageResponse, SharedChatSession, UserColor,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgesResponse {
    pub data: Vec<Badge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSettingResponse {
    pub data: Vec<ChatSetting>,
}

/// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
/// spec paginarion must include event empty.
/// but mock server remove paginiation field
#[derive(Debug, Serialize, Deserialize)]
pub struct ChattersResponse {
    pub data: Vec<Chatter>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotesResponse {
    pub data: Vec<Emote>,
    pub template: String,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendChatMessageResponse {
    pub data: Vec<MessageResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedChatSessionResponse {
    pub data: Vec<SharedChatSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersColorResponse {
    pub data: Vec<UserColor>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        chat::response::{
            BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse,
            SendChatMessageResponse, SharedChatSessionResponse, UsersColorResponse,
        },
        types::ModeratorId,
    };

    #[test]
    fn chatters_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "123456789",
                    "user_login": "testuser1",
                    "user_name": "TestUser1"
                },
                {
                    "user_id": "987654321",
                    "user_login": "testuser2",
                    "user_name": "TestUser2"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            },
            "total": 250
        });

        let response: ChattersResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.total, 250);
        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.unwrap().cursor, "eyJiI...");

        let first_chatter = &response.data[0];
        assert_eq!(first_chatter.user_id.as_str(), "123456789");
        assert_eq!(first_chatter.user_login, "testuser1");
        assert_eq!(first_chatter.user_name, "TestUser1");

        let second_chatter = &response.data[1];
        assert_eq!(second_chatter.user_id.as_str(), "987654321");
        assert_eq!(second_chatter.user_login, "testuser2");
        assert_eq!(second_chatter.user_name, "TestUser2");
    }

    #[test]
    fn emotes_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "emotesv2_123",
                    "name": "testEmote",
                    "images": {
                        "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/1.0",
                        "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/2.0",
                        "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/3.0"
                    },
                    "tier": "1000",
                    "emote_type": "subscriptions",
                    "emote_set_id": "0",
                    "format": ["static"],
                    "scale": ["1.0", "2.0", "3.0"],
                    "theme_mode": ["light", "dark"]
                }
            ],
            "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}",
            "pagination": {}
        });

        let response: EmotesResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.template.contains("{{id}}"));
        assert!(response.pagination.is_none());

        let emote = &response.data[0];
        assert_eq!(emote.id.as_str(), "emotesv2_123");
        assert_eq!(emote.name, "testEmote");
        assert!(emote.images.is_some());
        assert_eq!(emote.tier, Some("1000".to_string()));
    }

    #[test]
    fn chat_settings_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "emote_mode": false,
                    "follower_mode": true,
                    "follower_mode_duration": 600,
                    "moderator_id": "987654321",
                    "non_moderator_chat_delay": false,
                    "non_moderator_chat_delay_duration": 2,
                    "slow_mode": false,
                    "show_mode_wait_time": 30,
                    "subscriber_mode": false,
                    "unique_chat_mode": true
                }
            ]
        });

        let response: ChatSettingResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let settings = &response.data[0];
        assert_eq!(settings.broadcaster_id.as_str(), "123456789");
        assert!(!settings.emote_mode);
        assert!(settings.follower_mode);
        assert_eq!(settings.follower_mode_duration, Some(600));
        assert_eq!(settings.moderator_id, Some(ModeratorId::from("987654321")));
        assert!(settings.unique_chat_mode);
    }

    #[test]
    fn shared_chat_session_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "session_id": "session123",
                    "host_broadcaster_id": "123456789",
                    "participants": [
                        {
                            "broadcaster_id": "123456789"
                        },
                        {
                            "broadcaster_id": "987654321"
                        }
                    ],
                    "created_at": "2024-01-15T10:30:00Z",
                    "updated_at": "2024-01-15T11:30:00Z"
                }
            ]
        });

        let response: SharedChatSessionResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let session = &response.data[0];
        assert_eq!(session.session_id, "session123");
        assert_eq!(session.host_broadcaster_id, "123456789");
        assert_eq!(session.participants.len(), 2);
        assert_eq!(session.participants[0].broadcaster_id.as_str(), "123456789");
        assert_eq!(session.participants[1].broadcaster_id.as_str(), "987654321");
    }

    #[test]
    fn send_chat_message_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "message_id": "msg123456",
                    "is_sent": true,
                    "drop_reason": null
                }
            ]
        });

        let response: SendChatMessageResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let message = &response.data[0];
        assert_eq!(message.message_id, "msg123456");
        assert!(message.is_sent);
        assert!(message.drop_reason.is_none());
    }

    #[test]
    fn send_chat_message_response_with_drop_reason() {
        let json_data = json!({
            "data": [
                {
                    "message_id": "msg789",
                    "is_sent": false,
                    "drop_reason": {
                        "code": "msg_rejected",
                        "message": "Message was rejected by AutoMod"
                    }
                }
            ]
        });

        let response: SendChatMessageResponse = serde_json::from_value(json_data).unwrap();

        let message = &response.data[0];
        assert_eq!(message.message_id, "msg789");
        assert!(!message.is_sent);
        assert!(message.drop_reason.is_some());

        let drop_reason = message.drop_reason.as_ref().unwrap();
        assert_eq!(drop_reason.code, "msg_rejected");
        assert_eq!(drop_reason.message, "Message was rejected by AutoMod");
    }

    #[test]
    fn users_color_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "123456789",
                    "user_name": "TestUser",
                    "user_login": "testuser",
                    "color": "#FF0000"
                },
                {
                    "user_id": "987654321",
                    "user_name": "AnotherUser",
                    "user_login": "anotheruser",
                    "color": "#0000FF"
                }
            ]
        });

        let response: UsersColorResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_user = &response.data[0];
        assert_eq!(first_user.user_id.as_str(), "123456789");
        assert_eq!(first_user.color, "#FF0000");

        let second_user = &response.data[1];
        assert_eq!(second_user.user_id.as_str(), "987654321");
        assert_eq!(second_user.color, "#0000FF");
    }

    #[test]
    fn badge_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "set_id": "subscriber",
                    "versions": [
                        {
                            "id": "3",
                            "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/subscriber/3/1",
                            "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/subscriber/3/2",
                            "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/subscriber/3/3",
                            "title": "3-Month Subscriber",
                            "description": "Subscribed for 3 months",
                            "click_action": null,
                            "click_url": null
                        }
                    ]
                }
            ]
        });

        let response: BadgesResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let badge = &response.data[0];
        assert_eq!(badge.set_id, "subscriber");
        assert_eq!(badge.versions.len(), 1);

        let version = &badge.versions[0];
        assert_eq!(version.id.as_str(), "3");
        assert_eq!(version.title, "3-Month Subscriber");
        assert_eq!(version.description, "Subscribed for 3 months");
        assert!(version.click_action.is_none());
        assert!(version.click_url.is_none());
    }
}
