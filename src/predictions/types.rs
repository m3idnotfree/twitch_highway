use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    id: Id,
    broadcaster_id: BroadcasterId,
    broadcaster_name: String,
    broadcaster_login: String,
    title: String,
    winning_outcome_id: Option<String>,
    outcomes: Vec<PredictionOutComes>,
    prediction_window: u64,
    status: PredictionStatus,
    created_at: DateTime<FixedOffset>,
    ended_at: Option<DateTime<FixedOffset>>,
    locked_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionOutComes {
    pub id: Id,
    pub title: String,
    pub users: u64,
    pub channel_points: u64,
    pub top_predictors: Option<TopPredictor>,
    pub color: PredictionColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PredictionStatus {
    ACTIVE,
    CANCELED,
    LOCKED,
    RESOLVED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPredictor {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub channel_points_used: u64,
    pub channel_points_won: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PredictionColor {
    BLUE,
    PINK,
}
