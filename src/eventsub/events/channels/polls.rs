use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Choice, PollId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPollBegin {
    pub id: PollId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub choices: Vec<Choice>,
    // Not supported
    pub bits_voting: Option<BitsVoting>,
    pub channel_points_voting: ChannelPointsVoting,
    pub status: String,
    pub started_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPollProgress {
    pub id: PollId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub choices: Vec<Choice>,
    // Not supported
    pub bits_voting: Option<BitsVoting>,
    pub channel_points_voting: ChannelPointsVoting,
    pub started_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPollEnd {
    pub id: PollId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub choices: Vec<Choice>,
    // Not supported
    pub bits_voting: Option<BitsVoting>,
    pub channel_points_voting: ChannelPointsVoting,
    pub started_at: DateTime<FixedOffset>,
    pub ends_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsVoting {
    pub is_enabled: bool,
    pub amount_per_vote: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPointsVoting {
    pub is_enabled: bool,
    pub amount_per_vote: u32,
}
