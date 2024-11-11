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

#[derive(Debug, Serialize)]
pub struct CreateEventSub<'a> {
    #[serde(skip)]
    pub access_token: Arc<AccessToken>,
    #[serde(skip)]
    pub client_id: Arc<ClientId>,
    #[serde(skip)]
    pub url: &'a Url,
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub version: &'a str,
    pub condition: HashMap<&'a str, &'a str>,
    pub transport: Transport,
}
impl<'a> CreateEventSub<'a> {
    pub fn new(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        url: &'a Url,
        kind: &'a str,
        version: &'a str,
        condition: HashMap<&'a str, &'a str>,
        transport_method: TransportMethod,
    ) -> CreateEventSub<'a> {
        Self {
            access_token,
            client_id,
            url,
            kind,
            version,
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

    pub fn set_type(mut self, kind: &'a str) -> Self {
        self.kind = kind;
        self
    }
    pub fn set_version(mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }
    pub fn set_condition(mut self, condition: HashMap<&'a str, &'a str>) -> Self {
        self.condition = condition;
        self
    }
    pub fn set_transport(mut self, transport: Transport) -> Self {
        self.transport = transport;
        self
    }
}

impl APIRequest for CreateEventSub<'_> {
    fn json(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .content_type_json()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Client-Id", self.client_id.as_str())
            .unwrap()
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
