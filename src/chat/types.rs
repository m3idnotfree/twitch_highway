use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, Images, ModeratorId, UserId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct Emote {
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Images>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_type: Option<EmoteType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    pub format: Vec<Format>,
    pub scale: Vec<Scale>,
    pub theme_mode: Vec<ThemeMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSetting {
    pub broadcaster_id: BroadcasterId,
    pub emote_mode: bool,
    pub follower_mode: bool,
    pub follower_mode_duration: Option<u64>,
    /// only includes the moderator:read:chat_settings scope
    pub moderator_id: Option<ModeratorId>,
    /// only includes the moderator:read:chat_settings scope
    pub non_moderator_chat_delay: Option<bool>,
    /// The response includes this field only
    /// if the request specifies a user access token that includes the
    /// moderator:read:chat_settings scope
    /// and the user in the moderator_id query parameter is
    /// one of the broadcaster’s moderators.
    pub non_moderator_chat_delay_duration: Option<u64>,
    pub slow_mode: bool,
    pub show_mode_wait_time: Option<u64>,
    pub subscriber_mode: bool,
    pub unique_chat_mode: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chatter {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message_id: String,
    pub is_sent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_reason: Option<DropReason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropReason {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedChatSession {
    pub session_id: String,
    pub host_broadcaster_id: String,
    pub participants: Vec<BroadcasterIdField>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    pub created_at: DateTime<FixedOffset>,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcasterIdField {
    pub broadcaster_id: BroadcasterId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserColor {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub color: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmoteType {
    None,
    Bitstier,
    Follower,
    Subscriptions,
    ChannelPoints,
    Rewards,
    Hypetrain,
    Prime,
    Tupbo,
    Smilies,
    Globals,
    Owl2019,
    Twofactor,
    Limitedtime,
    #[cfg(feature = "mock-api")]
    Subscription,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Animated,
    Static,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Scale {
    #[serde(rename(serialize = "1.0", deserialize = "1.0"))]
    Small,
    #[serde(rename(serialize = "2.0", deserialize = "2.0"))]
    Medium,
    #[serde(rename(serialize = "3.0", deserialize = "3.0"))]
    Large,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: Id,
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub title: String,
    pub description: String,
    pub click_action: Option<String>,
    pub click_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::chat::types::{EmoteType, Format, Scale, ThemeMode};

    #[test]
    fn emote_type_serialization() {
        let emote_types = vec![
            EmoteType::None,
            EmoteType::Bitstier,
            EmoteType::Follower,
            EmoteType::Subscriptions,
            EmoteType::ChannelPoints,
            EmoteType::Rewards,
            EmoteType::Hypetrain,
            EmoteType::Prime,
            EmoteType::Tupbo,
            EmoteType::Smilies,
            EmoteType::Globals,
            EmoteType::Owl2019,
            EmoteType::Twofactor,
            EmoteType::Limitedtime,
        ];

        for emote_type in emote_types {
            let serialized = serde_json::to_string(&emote_type).unwrap();
            let deserialized: EmoteType = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn scale_enum() {
        let scales = vec![
            (Scale::Small, "\"1.0\""),
            (Scale::Medium, "\"2.0\""),
            (Scale::Large, "\"3.0\""),
        ];

        for (scale, expected_json) in scales {
            let serialized = serde_json::to_string(&scale).unwrap();
            assert_eq!(serialized, expected_json);

            let deserialized: Scale = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn format_and_theme_mode() {
        let formats = vec![Format::Animated, Format::Static];
        for format in formats {
            let serialized = serde_json::to_string(&format).unwrap();
            let deserialized: Format = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }

        let theme_modes = vec![ThemeMode::Dark, ThemeMode::Light];
        for theme_mode in theme_modes {
            let serialized = serde_json::to_string(&theme_mode).unwrap();
            let deserialized: ThemeMode = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
