use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, GameId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEditor {
    pub user_id: UserId,
    pub user_name: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedChannel {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub followed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFollower {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub followed_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentClassificationLabel {
    id: ContentClassificationLabelsID,
    is_enabled: bool,
}

impl ContentClassificationLabel {
    pub fn new(id: ContentClassificationLabelsID, is_enabled: bool) -> Self {
        Self { id, is_enabled }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentClassificationLabelsID {
    DebatedSocialIssuesAndPolitics,
    DrugsIntoxication,
    SexualThemes,
    ViolentGraphic,
    Gambling,
    ProfanityVulgarity,
}
