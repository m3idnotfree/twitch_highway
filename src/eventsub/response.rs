use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    eventsub::{Condition, SubscriptionType},
    types::{ConduitId, Pagination, SessionId, Status, SubscriptionId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventSubscriptionsResponse {
    pub data: Vec<EventSubData>,
    pub total_cost: u64,
    pub max_total_cost: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscriptionsResponse {
    pub data: Vec<EventSubData>,
    pub total_cost: u64,
    pub max_total_cost: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventSubData {
    pub id: SubscriptionId,
    pub status: Status,
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: Condition,
    pub created_at: DateTime<FixedOffset>,
    pub transport: TransportResponse,
    pub cost: u64,
}

impl<'de> serde::Deserialize<'de> for EventSubData {
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
            transport: TransportResponse,
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

        Ok(EventSubData {
            id: helper.id,
            status: helper.status,
            kind,
            version: helper.version,
            condition: helper.condition,
            created_at: helper.created_at,
            transport: helper.transport,
            cost: helper.cost,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
pub enum TransportResponse {
    Webhook {
        callback: Url,
    },
    Websocket {
        session_id: SessionId,
        connected_at: DateTime<FixedOffset>,
    },
    Conduit {
        conduit_id: ConduitId,
    },
}
