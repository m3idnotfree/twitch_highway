use std::str;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    eventsub::{resolve_subscription_type, websocket::MessageType, SubscriptionType},
    types::MessageId,
};

/// <https://dev.twitch.tv/docs/eventsub/websocket-reference>
#[derive(Debug, Clone, Serialize)]
pub struct MetaData {
    pub message_id: MessageId,
    pub message_type: MessageType,
    pub message_timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<SubscriptionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            message_id: MessageId,
            message_type: MessageType,
            message_timestamp: DateTime<FixedOffset>,
            subscription_type: Option<SubscriptionType>,
            subscription_version: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        #[rustfmt::skip]
        let subscription_type = helper.subscription_type.map(|kind| {
            resolve_subscription_type!(kind, opt helper.subscription_version.as_deref())
        });

        Ok(MetaData {
            message_id: helper.message_id,
            message_type: helper.message_type,
            message_timestamp: helper.message_timestamp,
            subscription_type,
            subscription_version: helper.subscription_version,
        })
    }
}
