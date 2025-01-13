use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    pub id: Id,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    #[serde(rename = "type")]
    pub kind: GoalType,
    pub description: String,
    pub current_amount: u64,
    pub target_amount: u64,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Follower,
    Subscription,
    SubscriptionCount,
    NewSubscription,
    NewSubscriptionCount,
}
