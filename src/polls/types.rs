use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: Id,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub title: String,
    pub choices: Vec<Choices>,
    pub bits_voting_enabled: bool,
    pub bits_per_vote: u64,
    pub channel_points_voting_enabled: bool,
    pub channel_points_per_vote: u64,
    pub status: PollStatus,
    pub duration: u64,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub id: Id,
    pub title: String,
    pub votes: u64,
    pub channel_points_votes: u64,
    pub bits_votes: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PollStatus {
    ACTIVE,
    COMPLETED,
    TERMINATED,
    ARCHIVED,
    MODERATED,
    INVALID,
}
