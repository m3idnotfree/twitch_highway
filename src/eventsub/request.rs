use asknothingx2_eventsub::twitch::{
    subscription_types::{
        channel_subscriptions::{
            drop_entitlement_grant::DropEntitlementGrantCondition, ChannelRaidRequest,
        },
        SubscriptionType,
    },
    Transport,
};
use serde::{Deserialize, Serialize};

use crate::AsBody;

use super::types::EventStatus;

impl AsBody for ChannelRaidRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubRequest {
    pub status: Option<EventStatus>,
    #[serde(rename = "type")]
    pub kind: Option<SubscriptionType>,
    pub user_id: Option<String>,
    pub after: Option<String>,
}

impl EventSubRequest {
    pub fn new(
        status: Option<EventStatus>,
        kind: Option<SubscriptionType>,
        user_id: Option<String>,
        after: Option<String>,
    ) -> Self {
        Self {
            status,
            kind,
            user_id,
            after,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DropEntitlementGrantConditionRequest {
    #[serde(rename = "type")]
    kind: SubscriptionType,
    version: String,
    condition: DropEntitlementGrantCondition,
    transport: Transport,
    is_batching_enabled: bool,
}

impl DropEntitlementGrantConditionRequest {
    pub fn new(
        kind: SubscriptionType,
        transport: Transport,
        condition: DropEntitlementGrantCondition,
    ) -> Self {
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
            is_batching_enabled: true,
        }
    }
}
