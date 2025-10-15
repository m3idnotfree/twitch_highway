use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, GameId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub broadcaster_language: String,
    pub broadcaster_login: String,
    pub display_name: String,
    pub game_id: GameId,
    pub game_name: String,
    pub id: BroadcasterId,
    pub is_live: bool,
    /// deprecated
    pub tag_ids: Vec<String>,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub title: String,
    pub started_at: String,
}
