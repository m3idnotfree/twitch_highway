use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ModeratorId, SessionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGuestStarSessionBegin {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub session_id: SessionId,
    pub started_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGuestStarSessionEnd {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub session_id: SessionId,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
    pub host_user_id: BroadcasterId,
    pub host_user_name: String,
    pub host_user_login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGuestStarGuestUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub session_id: SessionId,
    pub moderator_user_id: Option<ModeratorId>,
    pub moderator_user_login: Option<String>,
    pub moderator_user_name: Option<String>,
    pub guest_user_id: Option<BroadcasterId>,
    pub guest_user_name: Option<String>,
    pub guest_user_login: Option<String>,
    pub slot_id: Option<String>,
    pub state: Option<GuestStatus>,
    pub host_user_id: String,
    pub host_user_name: String,
    pub host_user_login: String,
    pub host_video_enabled: Option<bool>,
    pub host_audio_enabled: Option<bool>,
    pub host_volume: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGuestStarSettingsUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_name: String,
    pub broadcaster_user_login: String,
    pub is_moderator_send_live_enabled: bool,
    pub slot_count: u32,
    pub is_browser_source_audio_enabled: bool,
    pub group_layout: Layout,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuestStatus {
    Invited,
    Accepted,
    Ready,
    Backstage,
    Live,
    Removed,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Tiled,
    Screenshare,
    HorizontalTop,
    HorizontalBottom,
    VerticalLeft,
    VerticalRight,
}
