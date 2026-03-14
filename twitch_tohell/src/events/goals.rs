use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, GoalId};

use super::serde_helpers::deserialize_optional_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goals {
    pub id: GoalId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_name: String,
    pub broadcaster_user_login: String,
    #[serde(rename = "type")]
    pub kind: GoalType,
    pub description: String,
    pub is_achieved: bool,
    pub current_amount: u64,
    pub target_amount: u64,
    pub started_at: DateTime<FixedOffset>,
    #[serde(default, deserialize_with = "deserialize_optional_datetime")]
    pub ended_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Follow,
    Subscription,
    SubscriptionCount,
    NewSubscription,
    NewSubscriptionCount,
    NewBit,
    NewCheerer,
}
