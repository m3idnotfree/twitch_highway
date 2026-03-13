use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, EmoteId, Images, ModeratorId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct Emote {
    pub id: EmoteId,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub versions: Vec<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chatter {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message_id: String,
    pub is_sent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_reason: Option<DropReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropReason {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatSession {
    pub session_id: String,
    pub host_broadcaster_id: String,
    pub participants: Vec<BroadcasterIdField>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    pub created_at: DateTime<FixedOffset>,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterIdField {
    pub broadcaster_id: BroadcasterId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    // #[cfg(feature = "test")]
    Subscription,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Animated,
    Static,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Scale {
    #[serde(rename(serialize = "1.0", deserialize = "1.0"))]
    Small,
    #[serde(rename(serialize = "2.0", deserialize = "2.0"))]
    Medium,
    #[serde(rename(serialize = "3.0", deserialize = "3.0"))]
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ChatColor {
    Blue,
    BlueViolet,
    CadetBlue,
    Chocolate,
    Coral,
    DodgerBlue,
    Firebrick,
    GoldenRod,
    Green,
    HotPink,
    OrangeRed,
    Red,
    SeaGreen,
    SpringGreen,
    YellowGreen,
}

impl ChatColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blue => "blue",
            Self::BlueViolet => "blue_violet",
            Self::CadetBlue => "cadet_blue",
            Self::Chocolate => "chocolate",
            Self::Coral => "coral",
            Self::DodgerBlue => "dodger_blue",
            Self::Firebrick => "firebrick",
            Self::GoldenRod => "golden_rod",
            Self::Green => "green",
            Self::HotPink => "hot_pink",
            Self::OrangeRed => "orange_red",
            Self::Red => "red",
            Self::SeaGreen => "sea_green",
            Self::SpringGreen => "spring_green",
            Self::YellowGreen => "yellow_green",
        }
    }
}

impl AsRef<str> for ChatColor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnnouncementColor {
    Blue,
    Green,
    Orange,
    Purple,
    Primary,
}

impl AnnouncementColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Purple => "purple",
            Self::Primary => "primary",
        }
    }
}
