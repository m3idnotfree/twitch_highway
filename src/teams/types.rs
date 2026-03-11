use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{
    constants::{ID, NAME},
    BroadcasterId, TeamId, UserId,
};

pub enum TeamSelect<'a> {
    Name(&'a str),
    Id(&'a TeamId),
}

impl<'a> From<&'a str> for TeamSelect<'a> {
    fn from(value: &'a str) -> Self {
        Self::Name(value)
    }
}

impl<'a> From<&'a TeamId> for TeamSelect<'a> {
    fn from(value: &'a TeamId) -> Self {
        Self::Id(value)
    }
}

impl<'a> TeamSelect<'a> {
    pub(crate) fn append_to_query(&self, url: &mut url::Url) {
        match self {
            Self::Name(name) => {
                url.query_pairs_mut().append_pair(NAME, name);
            }
            Self::Id(id) => {
                url.query_pairs_mut().append_pair(ID, id);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub id: TeamId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub id: TeamId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUser {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
}
