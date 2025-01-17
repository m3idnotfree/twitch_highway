use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClip {
    pub edit_url: String,
    pub id: Id,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clip {
    pub id: Id,
    pub url: String,
    pub embed_url: String,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub creator_id: String,
    pub creator_name: String,
    pub video_id: String,
    pub game_id: String,
    pub language: String,
    pub title: String,
    pub view_count: u64,
    pub created_at: String,
    pub thumbnail_url: String,
    pub duration: f64,
    pub vod_offset: Option<u64>,
    pub is_featured: bool,
}
