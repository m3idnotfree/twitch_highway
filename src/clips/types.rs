use asknothingx2_util::serde::deserialize_empty_string_as_none;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{BroadcasterId, ClipId, GameId, UserId, VideoId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClip {
    pub edit_url: Url,
    pub id: ClipId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub url: Url,
    pub embed_url: Url,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub creator_id: UserId,
    pub creator_name: String,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub video_id: Option<VideoId>,
    pub game_id: GameId,
    pub language: String,
    pub title: String,
    pub view_count: u64,
    pub created_at: String,
    pub thumbnail_url: Url,
    pub duration: f64,
    pub vod_offset: Option<u64>,
    pub is_featured: bool,
}
