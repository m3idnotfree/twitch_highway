use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Choice, PollId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: PollId,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub title: String,
    pub choices: Vec<Choice>,
    pub bits_voting_enabled: bool,
    pub bits_per_vote: u64,
    pub channel_points_voting_enabled: bool,
    pub channel_points_per_vote: u64,
    pub status: PollStatus,
    pub duration: u64,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EndPollStatus {
    TERMINATED,
    ARCHIVED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PollStatus {
    ACTIVE,
    COMPLETED,
    TERMINATED,
    ARCHIVED,
    MODERATED,
    INVALID,
}

#[derive(Serialize)]
pub(crate) struct EndPollBody<'a> {
    pub broadcaster_id: &'a BroadcasterId,
    pub id: &'a PollId,
    pub status: EndPollStatus,
}

#[cfg(test)]
mod tests {
    use crate::polls::PollStatus;

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
