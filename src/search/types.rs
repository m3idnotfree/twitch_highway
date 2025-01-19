use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub broadcaster_language: String,
    pub broadcaster_login: String,
    pub display_name: String,
    pub game_id: String,
    pub game_name: String,
    pub id: Id,
    pub is_live: bool,
    /// deprecated
    pub tag_ids: Vec<String>,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub title: String,
    pub started_at: String,
}
