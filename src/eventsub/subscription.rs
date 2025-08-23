use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    eventsub::{condition::Condition, subscription_types::SubscriptionType},
    types::{Status, SubscriptionId},
};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#subscription>
#[derive(Debug, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub status: Status,
    pub cost: u64,
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
            kind: SubscriptionType,
            version: String,
            condition: Condition,
            created_at: DateTime<FixedOffset>,
            cost: u64,
        }

        let helper = Helper::deserialize(deserializer)?;

        let kind = match helper.kind {
            kind @ SubscriptionType::AutomodMessageHold => {
                if helper.version == "2" {
                    SubscriptionType::AutomodMessageHoldV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionType::AutomodMessageUpdate => {
                if helper.version == "2" {
                    SubscriptionType::AutomodMessageUpdateV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionType::ChannelModerate => {
                if helper.version == "2" {
                    SubscriptionType::ChannelModerateV2
                } else {
                    kind
                }
            }
            _ => helper.kind,
        };

        Ok(Subscription {
            id: helper.id,
            status: helper.status,
            kind,
            version: helper.version,
            condition: helper.condition,
            created_at: helper.created_at,
            cost: helper.cost,
        })
    }
}
