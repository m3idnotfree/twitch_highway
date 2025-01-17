use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::Id;

//#[derive(Debug, Serialize, Deserialize)]
//pub struct Segment {
//    pub segments: Vec<ScheduleSegment>,
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: Id,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub title: String,
    pub canceled_until: Option<String>,
    pub category: Option<Category>,
    pub is_recurring: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vacation {
    pub start_time: String,
    pub end_time: String,
}
