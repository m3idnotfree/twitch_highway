use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize, de::Error as DeError};

use crate::{
    eventsub::{Condition, Status, SubscriptionType},
    ids::SubscriptionId,
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

impl<'de> Deserialize<'de> for Subscription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            id: SubscriptionId,
            status: Status,
            #[serde(rename = "type")]
            subscription_type: String,
            version: String,
            condition: Condition,
            created_at: DateTime<FixedOffset>,
            cost: u64,
            transport: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;

        let subscription_type = SubscriptionType::from_type_and_version(
            &helper.subscription_type,
            Some(helper.version.as_ref()),
        )
        .map_err(DeError::custom)?;

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
