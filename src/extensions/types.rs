use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Cost, Id};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Segment {
    Broadcaster,
    Developer,
    Global,
}

impl AsRef<str> for Segment {
    fn as_ref(&self) -> &str {
        match self {
            Self::Broadcaster => "broadcaster",
            Self::Developer => "developer",
            Self::Global => "global",
        }
    }
}

impl From<Segment> for String {
    fn from(value: Segment) -> Self {
        value.as_ref().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSegment {
    pub segment: Segment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_id: Option<BroadcasterId>,
    pub content: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub game_name: String,
    pub game_id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretData {
    pub format_version: u64,
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub content: String,
    pub active_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    pub author_name: String,
    pub bits_enabled: bool,
    pub can_install: bool,
    pub configuration_location: ConfigurationLocation,
    pub description: String,
    pub eula_tos_url: String,
    pub has_chat_support: bool,
    pub icon_url: String,
    pub icon_urls: HashMap<String, String>,
    pub id: Id,
    pub name: String,
    pub privacy_policy_url: String,
    pub request_identity_link: bool,
    pub screenshot_urls: Vec<String>,
    pub state: State,
    pub subscriptions_support_level: SubscriptionsSupportLevel,
    pub summary: String,
    pub support_email: String,
    pub version: String,
    pub viewer_summary: String,
    pub views: Views,
    pub allowlisted_config_urls: Vec<String>,
    pub allowlisted_panel_urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigurationLocation {
    Hosted,
    Custom,
    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum State {
    Approved,
    AssetsUploaded,
    Deleted,
    Deprecated,
    InReview,
    InTest,
    PendingAction,
    Rejected,
    Released,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionsSupportLevel {
    None,
    Optional,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Views {
    pub mobile: Mobile,
    pub panel: Panel,
    pub video_overlay: VideoOverlay,
    pub component: Component,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mobile {
    pub viewer_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Panel {
    pub viewer_url: String,
    pub height: u64,
    pub can_link_external_content: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOverlay {
    pub viewer_url: String,
    pub can_link_external_content: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub viewer_url: String,
    pub aspect_width: u64,
    pub aspect_height: u64,
    pub aspect_ratio_x: u64,
    pub aspect_ratio_y: u64,
    pub autoscale: bool,
    pub scale_pixels: u64,
    pub target_height: u64,
    pub size: u64,
    pub zoom: bool,
    pub zoom_pixels: u64,
    pub can_link_external_content: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitsProductExtension {
    pub sku: String,
    pub cost: Cost,
    pub display_name: String,
    pub in_development: bool,
    pub expiration: DateTime<FixedOffset>,
    pub is_broadcast: bool,
}

impl BitsProductExtension {
    pub fn into_json(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::types::{
        ConfigurationLocation, Segment, State, SubscriptionsSupportLevel,
    };

    #[test]
    fn segment_enum() {
        let segments = vec![
            (Segment::Broadcaster, "broadcaster"),
            (Segment::Developer, "developer"),
            (Segment::Global, "global"),
        ];

        for (segment, expected_str) in segments {
            assert_eq!(segment.as_ref(), expected_str);

            let segment_string: String = segment.into();
            assert_eq!(segment_string, expected_str);

            let serialized = serde_json::to_string(&segment).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: Segment = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_ref(), expected_str);
        }
    }

    #[test]
    fn configuration_location_enum() {
        let locations = vec![
            ConfigurationLocation::Hosted,
            ConfigurationLocation::Custom,
            ConfigurationLocation::None,
        ];

        for location in locations {
            let serialized = serde_json::to_string(&location).unwrap();
            let deserialized: ConfigurationLocation = serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn state_enum() {
        let states = vec![
            State::Approved,
            State::AssetsUploaded,
            State::Deleted,
            State::Deprecated,
            State::InReview,
            State::InTest,
            State::PendingAction,
            State::Rejected,
            State::Released,
        ];

        for state in states {
            let serialized = serde_json::to_string(&state).unwrap();
            let deserialized: State = serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn subscriptions_support_level_enum() {
        let levels = vec![
            SubscriptionsSupportLevel::None,
            SubscriptionsSupportLevel::Optional,
        ];

        for level in levels {
            let serialized = serde_json::to_string(&level).unwrap();
            let deserialized: SubscriptionsSupportLevel =
                serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
