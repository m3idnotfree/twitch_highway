use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub segments: Vec<ScheduleSegment>,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub vacation: Option<Vacation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleSegment {
    pub id: Id,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub title: String,
    pub canceled_until: Option<String>,
    pub category: Option<ScheduleCategory>,
    pub is_recurring: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleCategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vacation {
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}
