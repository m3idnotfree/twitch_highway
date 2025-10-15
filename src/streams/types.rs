use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{GameId, MarkerId, UserId, VideoId};

#[derive(Debug, Clone, Deserialize)]
pub struct StreamKey {
    pub stream_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub id: VideoId,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub game_id: Option<GameId>,
    pub game_name: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub title: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    pub viewer_count: u64,
    pub started_at: DateTime<FixedOffset>,
    pub language: String,
    pub thumbnail_url: Option<String>,
    // deprecated
    #[serde(default)]
    pub tag_ids: Option<Vec<String>>,
    pub is_mature: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMarker {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub videos: Vec<StreamVideos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamVideos {
    pub video_id: VideoId,
    pub markers: Vec<Marker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub id: MarkerId,
    pub created_at: DateTime<FixedOffset>,
    pub position_seconds: u64,
    pub description: String,
    #[serde(alias = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Serialize)]
pub(crate) struct CreateStrseamMarkerBody<'a> {
    pub user_id: &'a UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}
