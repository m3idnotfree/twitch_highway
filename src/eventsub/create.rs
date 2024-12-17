use std::{collections::HashMap, sync::Arc};

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum EventSubCreateError {
    #[error("empty value: {0}")]
    EmptyValue(String),
}

/// https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription
#[derive(Debug, Serialize)]
pub struct CreateEventSub {
    #[serde(skip)]
    access_token: Arc<AccessToken>,
    #[serde(skip)]
    client_id: Arc<ClientId>,
    #[serde(skip)]
    url: Url,
    #[serde(rename = "type")]
    kind: String,
    version: String,
    condition: HashMap<String, String>,
    transport: Transport,
}

impl CreateEventSub {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        kind: T,
        version: T,
        condition: HashMap<String, String>,
        transport_method: TransportMethod,
    ) -> CreateEventSub {
        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push("eventsub")
            .push("subscriptions");

        Self {
            access_token,
            client_id,
            url,
            kind: kind.into(),
            version: version.into(),
            condition,
            transport: Transport {
                method: transport_method,
                callback: None,
                secret: None,
                session_id: None,
                conduit_id: None,
            },
        }
    }

    pub fn set_type<T: Into<String>>(mut self, kind: T) -> Self {
        self.kind = kind.into();
        self
    }

    pub fn set_version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
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

impl APIRequest for CreateEventSub {
    fn json(&self) -> Option<String> {
        Some(Self::json_to_string(self).unwrap())
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .content_type_json()
            .authorization("Bearer", self.access_token.secret().as_str())
            .client_id(self.client_id.as_str())
            .build()
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        self.url.clone()
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
    Conduit,
    Unknown,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use asknothingx2_util::{
        api::APIRequest,
        oauth::{AccessToken, ClientId},
    };
    use pretty_assertions::assert_eq;

    use super::CreateEventSub;
    use crate::{expect_APIRequest, expect_headers, TransportMethod};

    #[test]
    fn create_eventsub() {
        let mut condition = HashMap::new();
        condition.insert("user_id", "1234");

        let create_eventsub = CreateEventSub::new(
            Arc::new(AccessToken::new(
                "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
            )),
            Arc::new(ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
            "user.update",
            "1",
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
