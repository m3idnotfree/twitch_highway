use std::collections::HashMap;

use asknothingx2_util::api::APIRequest;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{SubscriptionTypes, Transport};

#[derive(Debug, thiserror::Error)]
pub enum EventSubCreateError {
    #[error("empty value: {0}")]
    EmptyValue(String),
}

crate::impl_endpoint!(
    /// https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription
    CreateEventSub {
        kind: SubscriptionTypes,
        condition: HashMap<String, String>,
        transport: Transport
    };
    new = {
        params = {
            kind: SubscriptionTypes,
            condition: HashMap<String, String>,
            transport: Transport
        },
        init = {
            kind: kind,
            condition: condition,
            transport: transport
        }
    },
    url = ["eventsub", "subscriptions"]
);

impl CreateEventSub {
    pub fn set_type(mut self, kind: SubscriptionTypes) -> Self {
        self.kind = kind;
        self
    }

    pub fn set_condition<K, V, H: IntoIterator<Item = (K, V)>>(mut self, condition: H) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.condition
            .extend(condition.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn set_transport(mut self, transport: Transport) -> Self {
        self.transport = transport;
        self
    }
}

#[derive(Debug, Serialize)]
pub struct CreateEventRequest {
    #[serde(rename = "type")]
    kind: SubscriptionTypes,
    version: String,
    condition: HashMap<String, String>,
    transport: Transport,
}

impl CreateEventRequest {
    pub fn new(
        kind: SubscriptionTypes,
        condition: HashMap<String, String>,
        transport: Transport,
    ) -> Self {
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
        }
    }
}

impl APIRequest for CreateEventSub {
    crate::impl_api_request_method!(POST);
    crate::impl_api_request_header!(json);
    fn json(&self) -> Option<String> {
        let request = CreateEventRequest::new(
            self.kind.clone(),
            self.condition.clone(),
            self.transport.clone(),
        );

        Some(Self::json_to_string(&request).unwrap())
    }

    fn url(&self) -> Url {
        self.get_url()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubCreateResponse {
    data: EventSubCreateResponseData,
    total: u64,
    total_cost: u64,
    max_total_cost: u64,
}

#[derive(Debug, Serialize)]
pub struct EventSubCreateResponseData {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub kind: SubscriptionTypes,
    pub version: String,
    pub condition: HashMap<String, String>,
    pub created_at: DateTime<FixedOffset>,
    pub transport: Transport,
    pub cost: u64,
}

impl<'de> Deserialize<'de> for EventSubCreateResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            id: String,
            status: String,
            #[serde(rename = "type")]
            kind: SubscriptionTypes,
            version: String,
            condition: HashMap<String, String>,
            created_at: DateTime<FixedOffset>,
            transport: Transport,
            cost: u64,
        }

        let helper = Helper::deserialize(deserializer)?;

        let kind = match helper.kind {
            kind @ SubscriptionTypes::AutomodMessageHold => {
                if helper.version == "2" {
                    SubscriptionTypes::AutomodMessageHoldV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionTypes::AutomodMessageUpdate => {
                if helper.version == "2" {
                    SubscriptionTypes::AutomodMessageUpdateV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionTypes::ChannelModerate => {
                if helper.version == "2" {
                    SubscriptionTypes::ChannelModerateV2
                } else {
                    kind
                }
            }
            _ => helper.kind,
        };

        Ok(EventSubCreateResponseData {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use asknothingx2_util::oauth::{AccessToken, ClientId};
    use pretty_assertions::assert_eq;

    use super::CreateEventSub;
    use crate::{expect_APIRequest, expect_headers, SubscriptionTypes, Transport};

    #[test]
    fn create_eventsub() {
        let mut condition = HashMap::new();
        condition.insert("user_id", "1234");

        let create_eventsub = CreateEventSub::new(
            AccessToken::new("cfabdegwdoklmawdzdo98xt2fo512y".to_string()),
            ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string()),
            SubscriptionTypes::UserUpdate,
            HashMap::new(),
            Transport::websocket(""),
        )
        .set_condition(condition);

        let expected_headers = expect_headers!(json);
        let expected_json ="{\"type\":\"user.update\",\"version\":\"1\",\"condition\":{\"user_id\":\"1234\"},\"transport\":{\"method\":\"websocket\",\"session_id\":\"\"}}".to_string();

        expect_APIRequest!(
            POST,
            expected_headers,
            "https://api.twitch.tv/helix/eventsub/subscriptions",
            json = Some(expected_json),
            text = None,
            urlencoded = None,
            create_eventsub
        );
    }
}
