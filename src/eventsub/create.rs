use std::collections::HashMap;

use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::SubscriptionTypes;

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
            transport_method: TransportMethod
        },
        init = {
            kind: kind,
            condition: condition,
            transport: Transport {
                method: transport_method,
                callback: None,
                secret: None,
                session_id: None,
                conduit_id: None
            }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubCreateResponseData {
    id: String,
    status: String,
    #[serde(rename = "type")]
    kind: String,
    version: String,
    condition: HashMap<String, String>,
    created_at: String,
    transport: Transport,
    cost: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transport {
    pub method: TransportMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conduit_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
    Conduit,
    Unknown,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use asknothingx2_util::oauth::{AccessToken, ClientId};
    use pretty_assertions::assert_eq;

    use super::CreateEventSub;
    use crate::{expect_APIRequest, expect_headers, SubscriptionTypes, TransportMethod};

    #[test]
    fn create_eventsub() {
        let mut condition = HashMap::new();
        condition.insert("user_id", "1234");

        let create_eventsub = CreateEventSub::new(
            AccessToken::new("cfabdegwdoklmawdzdo98xt2fo512y".to_string()),
            ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string()),
            SubscriptionTypes::UserUpdate,
            HashMap::new(),
            TransportMethod::Websocket,
        )
        .set_condition(condition);

        let expected_headers = expect_headers!(json);
        let expected_json ="{\"type\":\"user.update\",\"version\":\"1\",\"condition\":{\"user_id\":\"1234\"},\"transport\":{\"method\":\"websocket\"}}".to_string();

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
