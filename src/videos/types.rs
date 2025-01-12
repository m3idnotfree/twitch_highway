use std::fmt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: Id,
    pub stream_id: Option<String>,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<FixedOffset>,
    pub published_at: DateTime<FixedOffset>,
    pub url: String,
    pub thumbnail_url: String,
    pub viewable: String,
    pub view_count: u64,
    pub language: String,
    #[serde(rename = "type")]
    pub kind: VideoType,
    pub duration: String,
    pub muted_segments: Vec<MutedSegments>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MutedSegments {
    pub duration: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    All,
    Day,
    Month,
    Week,
}

impl VideoPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl fmt::Display for VideoPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoSort {
    Time,
    Trending,
    Views,
}

impl VideoSort {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Time => "time",
            Self::Trending => "trending",
            Self::Views => "views",
        }
    }
}

impl fmt::Display for VideoSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoType {
    All,
    Archive,
    Highlight,
    Upload,
}

impl VideoType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Archive => "archive",
            Self::Highlight => "highlight",
            Self::Upload => "upload",
        }
    }
}

impl fmt::Display for VideoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
