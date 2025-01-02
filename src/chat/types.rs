use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct Emote {
    pub id: String,
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
    pub format: Vec<EmoteFormat>,
    pub scale: Vec<EmoteScale>,
    pub theme_mode: Vec<EmoteThemeMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSetting {
    pub broadcaster_id: String,
    pub emote_mode: bool,
    pub follower_mode: bool,
    pub follower_mode_duration: Option<u64>,
    /// only includes the moderator:read:chat_settings scope
    pub moderator_id: Option<String>,
    /// only includes the moderator:read:chat_settings scope
    pub non_moderator_chat_delay: bool,
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
    pub user_id: String,
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
    pub participants: Vec<BroadcasterId>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    pub created_at: DateTime<FixedOffset>,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcasterId {
    pub broadcaster_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserColor {
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmoteFormat {
    Animated,
    Static,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmoteThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmoteScale {
    #[serde(rename(serialize = "1.0", deserialize = "1.0"))]
    Small,
    #[serde(rename(serialize = "2.0", deserialize = "2.0"))]
    Medium,
    #[serde(rename(serialize = "3.0", deserialize = "3.0"))]
    Large,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub url_1x: String,
    pub url_2x: String,
    pub url_4x: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub title: String,
    pub description: String,
    pub click_action: Option<String>,
    pub click_url: Option<String>,
}
