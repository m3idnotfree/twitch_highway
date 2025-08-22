use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarSetting {
    pub is_moderator_send_live_enabled: bool,
    pub slot_count: Option<u64>,
    pub is_browser_source_audio_enabled: Option<bool>,
    pub group_layout: Option<GroupLayout>,
    pub browser_source_token: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupLayout {
    TILED_LAYOUT,
    SCREENSHARE_LAYOUT,
    HORIZONTAL_LAYOUT,
    VERTICAL_LAYOUT,
}

impl GroupLayout {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TILED_LAYOUT => "TILED_LAYOUT",
            Self::SCREENSHARE_LAYOUT => "SCREENSHARE_LAYOUT",
            Self::HORIZONTAL_LAYOUT => "HORIZONTAL_LAYOUT",
            Self::VERTICAL_LAYOUT => "VERTICAL_LAYOUT",
        }
    }
}

impl AsRef<str> for GroupLayout {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<GroupLayout> for String {
    fn from(value: GroupLayout) -> Self {
        match value {
            layout @ GroupLayout::TILED_LAYOUT => layout.as_str().to_string(),
            layout @ GroupLayout::SCREENSHARE_LAYOUT => layout.as_str().to_string(),
            layout @ GroupLayout::HORIZONTAL_LAYOUT => layout.as_str().to_string(),
            layout @ GroupLayout::VERTICAL_LAYOUT => layout.as_str().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarSession {
    pub id: Id,
    pub guests: Vec<Guest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guest {
    pub slot_id: Option<String>,
    pub user_id: UserId,
    pub user_display_name: String,
    pub user_login: String,
    pub is_live: bool,
    pub volume: u64,
    pub assigned_at: DateTime<FixedOffset>,
    pub audio_settings: GuestSetting,
    pub video_settings: GuestSetting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestSetting {
    pub is_available: bool,
    pub is_host_enabled: bool,
    pub is_guest_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestStarInvite {
    pub user_id: UserId,
    pub invited_at: String,
    pub status: GuestStarStatus,
    pub is_audio_enabled: bool,
    pub is_video_enabled: bool,
    pub is_audio_available: bool,
    pub is_video_available: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuestStarStatus {
    INVITED,
    ACCEPTED,
    READY,
}

#[cfg(test)]
mod tests {
    use crate::guest_star::types::{GroupLayout, GuestStarStatus};

    #[test]
    fn group_layout_enum() {
        let layouts = vec![
            (GroupLayout::TILED_LAYOUT, "TILED_LAYOUT"),
            (GroupLayout::SCREENSHARE_LAYOUT, "SCREENSHARE_LAYOUT"),
            (GroupLayout::HORIZONTAL_LAYOUT, "HORIZONTAL_LAYOUT"),
            (GroupLayout::VERTICAL_LAYOUT, "VERTICAL_LAYOUT"),
        ];

        for (layout, expected_str) in layouts {
            assert_eq!(layout.as_str(), expected_str);
            assert_eq!(layout.as_ref(), expected_str);

            let layout_string: String = layout.into();
            assert_eq!(layout_string, expected_str);

            let serialized = serde_json::to_string(&layout).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: GroupLayout = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn guest_star_status_enum() {
        let statuses = vec![
            GuestStarStatus::INVITED,
            GuestStarStatus::ACCEPTED,
            GuestStarStatus::READY,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: GuestStarStatus = serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
