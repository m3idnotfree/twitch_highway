use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrain {
    pub id: Id,
    pub event_type: String,
    pub event_timestamp: DateTime<FixedOffset>,
    pub version: String,
    pub event_data: HypeTrainEvent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainEvent {
    pub broadcaster_id: BroadcasterId,
    pub cooldown_end_time: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    pub goal: u64,
    pub id: Id,
    pub last_contribution: HypeTrainContribution,
    pub level: u64,
    pub started_at: DateTime<FixedOffset>,
    pub top_contributions: Vec<HypeTrainContribution>,
    pub total: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainContribution {
    pub total: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub user: String,
}
