use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcasterTeam {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub background_image_url: Option<String>,
    pub banner: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub info: String,
    pub thumbnail_url: String,
    pub team_name: String,
    pub team_display_name: String,
    pub id: Id,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub users: Vec<TeamUser>,
    pub background_image_url: Option<String>,
    pub banner: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub info: String,
    pub thumbnail_url: String,
    pub team_name: String,
    pub team_display_name: String,
    pub id: Id,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamUser {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
}
