use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Outcome, PredictionId, Title};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: PredictionId,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<Outcome>,
    pub prediction_window: u64,
    pub status: PredictionStatus,
    pub created_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
    pub locked_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionStatus {
    ACTIVE,
    CANCELED,
    LOCKED,
    RESOLVED,
}

impl PredictionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ACTIVE => "ACTIVE",
            Self::CANCELED => "CANCELED",
            Self::LOCKED => "LOCKED",
            Self::RESOLVED => "RESOLVED",
        }
    }
}

impl AsRef<str> for PredictionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Serialize)]
pub(crate) struct CreatePredictionBody<'a> {
    pub broadcaster_id: &'a BroadcasterId,
    pub title: &'a str,
    pub outcomes: &'a [Title],
    pub prediction_window: u64,
}
