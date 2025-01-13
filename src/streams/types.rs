use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{Id, UserId};

#[derive(Debug, Deserialize)]
pub struct StreamKey {
    pub stream_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub id: Id,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub title: String,
    pub tags: Vec<String>,
    pub viewer_count: u64,
    pub started_at: DateTime<FixedOffset>,
    pub language: String,
    pub thumbnail_url: String,
    // deprecated
    pub tag_ids: Option<Vec<String>>,
    pub is_mature: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub id: Id,
    pub created_at: DateTime<FixedOffset>,
    pub position_seconds: u64,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub URL: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMarker {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub videos: Vec<StreamVideos>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamVideos {
    pub video_id: String,
    pub markers: Vec<Marker>,
}
