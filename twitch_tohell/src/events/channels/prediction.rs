use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, Outcome, PredictionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionBegin {
    pub id: PredictionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<Outcome>,
    pub started_at: DateTime<FixedOffset>,
    pub locks_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionProgress {
    pub id: PredictionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<Outcome>,
    pub started_at: DateTime<FixedOffset>,
    pub locks_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionLock {
    pub id: PredictionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<Outcome>,
    pub started_at: DateTime<FixedOffset>,
    pub locks_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionEnd {
    pub id: PredictionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub winning_outcome_id: String,
    pub outcomes: Vec<Outcome>,
    pub status: PredictionStatus,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PredictionStatus {
    Resolved,
    Canceled,
}
