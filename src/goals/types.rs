use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, GoalId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Follower,
    Subscription,
    SubscriptionCount,
    NewSubscription,
    NewSubscriptionCount,
    // #[cfg(feature = "test")]
    Subscriber,
}
