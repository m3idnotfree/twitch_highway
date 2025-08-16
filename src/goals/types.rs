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
    #[cfg(feature = "test")]
    Subscriber,
}

#[cfg(test)]
mod tests {
    use crate::goals::types::GoalType;

    #[test]
    fn goal_type_enum() {
        let goal_types = vec![
            (GoalType::Follower, "follower"),
            (GoalType::Subscription, "subscription"),
            (GoalType::SubscriptionCount, "subscription_count"),
            (GoalType::NewSubscription, "new_subscription"),
            (GoalType::NewSubscriptionCount, "new_subscription_count"),
            #[cfg(feature = "test")]
            (GoalType::Subscriber, "subscriber"),
        ];

        for (goal_type, expected_str) in goal_types {
            let serialized = serde_json::to_string(&goal_type).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: GoalType = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
