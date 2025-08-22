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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollStatus {
    ACTIVE,
    COMPLETED,
    TERMINATED,
    ARCHIVED,
    MODERATED,
    INVALID,
}

#[cfg(test)]
mod tests {
    use crate::polls::types::PollStatus;

    #[test]
    fn poll_status_enum() {
        let statuses = vec![
            PollStatus::ACTIVE,
            PollStatus::COMPLETED,
            PollStatus::TERMINATED,
            PollStatus::ARCHIVED,
            PollStatus::MODERATED,
            PollStatus::INVALID,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: PollStatus = serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
