use serde::Serialize;

use crate::{
    conduits::{GetConduitShardsResponse, UpdateConduitShardsResponse},
    request::TwitchAPIRequest,
    types::{
        constants::{AFTER, CONDUITS, CONDUIT_ID, EVENTSUB, SHARDS, STATUS},
        ConduitId, SessionId, ShardId, Status,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetConduitShardsBuilder<'a> {
        req: {conduit_id: &'a ConduitId [key = CONDUIT_ID]},
        opts: {
            status: Status [key = STATUS, convert = as_ref],
            after: &'a str [key = AFTER]
    }
    } -> GetConduitShardsResponse;
    endpoint_type: GetConduitShards,
    method: GET,
    path: [EVENTSUB, CONDUITS, SHARDS],
}

#[derive(Debug, Serialize)]
pub struct UpdateConduitShardsBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    conduit_id: ConduitId,
    shards: Vec<ShardUpdate>,
}

impl<'a> UpdateConduitShardsBuilder<'a> {
    pub fn new(api: &'a TwitchAPI, conduit_id: ConduitId) -> Self {
        Self {
            api,
            conduit_id,
            shards: Vec::new(),
        }
    }

    pub fn webhook(
        mut self,
        id: ShardId,
        callback: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        self.shards
            .push(ShardUpdate::webhook(id, callback.into(), secret.into()));
        self
    }

    pub fn websocket(mut self, id: ShardId, session_id: SessionId) -> Self {
        self.shards.push(ShardUpdate::websocket(id, session_id));
        self
    }

    pub fn build(self) -> TwitchAPIRequest<UpdateConduitShardsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, CONDUITS, SHARDS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateConduitShards,
            url,
            reqwest::Method::PATCH,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<UpdateConduitShardsResponse, crate::Error> {
        self.build().json().await
    }
}

#[derive(Debug, Serialize)]
pub struct ShardUpdate {
    pub id: ShardId,
    pub transport: TransportConfig,
}

impl ShardUpdate {
    fn webhook(id: ShardId, callback: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            id,
            transport: TransportConfig::Webhook {
                callback: callback.into(),
                secret: secret.into(),
            },
        }
    }

    fn websocket(id: ShardId, session_id: SessionId) -> Self {
        Self {
            id,
            transport: TransportConfig::Websocket { session_id },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "method", rename_all = "lowercase")]
pub enum TransportConfig {
    Webhook { callback: String, secret: String },
    Websocket { session_id: SessionId },
}
