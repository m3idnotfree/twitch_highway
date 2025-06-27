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
    pub kind: Type,
    pub duration: String,
    pub muted_segments: Vec<MutedSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MutedSegment {
    pub duration: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    All,
    Day,
    Month,
    Week,
}

impl Period {
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for Period {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Time,
    Trending,
    Views,
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Time => "time",
            Self::Trending => "trending",
            Self::Views => "views",
        }
    }
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    All,
    Archive,
    Highlight,
    Upload,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Archive => "archive",
            Self::Highlight => "highlight",
            Self::Upload => "upload",
        }
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
