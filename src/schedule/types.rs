use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{Category, SegmentId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub title: String,
    pub canceled_until: Option<String>,
    pub category: Option<Category>,
    pub is_recurring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vacation {
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}
