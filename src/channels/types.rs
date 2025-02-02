use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, GameId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub broadcaster_language: String,
    pub game_id: GameId,
    pub game_name: String,
    pub title: String,
    pub delay: u64,
    pub tags: Vec<String>,
    pub content_classification_labels: Vec<String>,
    pub is_branded_content: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelEditor {
    pub user_id: UserId,
    pub user_name: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowedChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub followed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelFollower {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub followed_at: DateTime<FixedOffset>,
}
