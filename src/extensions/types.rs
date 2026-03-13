use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{BroadcasterId, Cost, ExtensionId, GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSegment {
    pub segment: Segment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_id: Option<BroadcasterId>,
    pub content: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub game_name: String,
    pub game_id: GameId,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretData {
    pub format_version: u64,
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub content: String,
    pub active_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub author_name: String,
    pub bits_enabled: bool,
    pub can_install: bool,
    pub configuration_location: ConfigurationLocation,
    pub description: String,
    pub eula_tos_url: String,
    pub has_chat_support: bool,
    pub icon_url: Url,
    pub icon_urls: HashMap<String, String>,
    pub id: ExtensionId,
    pub name: String,
    pub privacy_policy_url: String,
    pub request_identity_link: bool,
    pub screenshot_urls: Vec<Url>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigurationLocation {
    Hosted,
    Custom,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionsSupportLevel {
    None,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Views {
    pub mobile: Mobile,
    pub panel: Panel,
    pub video_overlay: VideoOverlay,
    pub component: Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mobile {
    pub viewer_url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub viewer_url: Url,
    pub height: u64,
    pub can_link_external_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOverlay {
    pub viewer_url: Url,
    pub can_link_external_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub viewer_url: Url,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Serialize)]
pub struct SetExtensionRequiredConfigurationBody<'a> {
    pub extension_id: &'a ExtensionId,
    pub extension_version: &'a str,
    pub required_configuration: &'a str,
}

#[derive(Serialize)]
pub struct SendExtensionPubSubMessageBody<'a> {
    pub target: &'a [&'a str],
    pub message: &'a str,
    pub broadcaster_id: &'a BroadcasterId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global_broadcast: Option<bool>,
}

#[derive(Serialize)]
pub struct ExtensionChatMessageIntoRequestBody<'a> {
    pub text: &'a str,
    pub extension_id: &'a ExtensionId,
    pub extension_version: &'a str,
}
