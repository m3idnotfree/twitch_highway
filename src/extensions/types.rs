use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Cost, Id};

#[derive(Debug, Serialize, Deserialize)]
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
    author_name: String,
    bits_enabled: bool,
    can_install: bool,
    configuration_location: ConfigurationLocation,
    description: String,
    eula_tos_url: String,
    has_chat_support: bool,
    icon_url: String,
    icon_urls: HashMap<String, String>,
    id: Id,
    name: String,
    privacy_policy_url: String,
    request_identity_link: bool,
    screenshot_urls: Vec<String>,
    state: State,
    subscriptions_support_level: SubscriptionsSupportLevel,
    summary: String,
    support_email: String,
    version: String,
    viewer_summary: String,
    views: Views,
    allowlisted_config_urls: Vec<String>,
    allowlisted_panel_urls: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigurationLocation {
    Hosted,
    Custom,
    None,
}
#[derive(Debug, Serialize, Deserialize)]
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
    pub expiration: String,
    pub is_broadcast: bool,
}

impl BitsProductExtension {
    pub fn to_json(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
