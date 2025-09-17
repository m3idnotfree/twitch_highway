use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::{
    eventsub::{Condition, SubscriptionType},
    types::{Status, SubscriptionId},
};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#subscription>
#[derive(Debug, Clone, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub status: Status,
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub cost: u64,
    pub transport: serde_json::Value,
    pub condition: Condition,
    pub created_at: DateTime<FixedOffset>,
}

impl<'de> serde::Deserialize<'de> for Subscription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper {
            id: SubscriptionId,
            status: Status,
            #[serde(rename = "type")]
            subscription_type: SubscriptionType,
            version: String,
            condition: Condition,
            created_at: DateTime<FixedOffset>,
            cost: u64,
            transport: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;

        let subscription_type =
            resolve_subscription_type!(helper.subscription_type, helper.version.as_ref());

        Ok(Subscription {
            id: helper.id,
            status: helper.status,
            kind: subscription_type,
            version: helper.version,
            condition: helper.condition,
            created_at: helper.created_at,
            cost: helper.cost,
            transport: helper.transport,
        })
    }
}
