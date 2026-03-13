use serde::Serialize;

use crate::{
    conduits::{GetConduitShardsResponse, UpdateConduitShardsResponse},
    types::{
        constants::{AFTER, CONDUITS, CONDUIT_ID, EVENTSUB, SHARDS, STATUS},
        ConduitId, SessionId, ShardId, Status,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetConduitShards<'a> {
    client: &'a Client,
    conduit_id: &'a ConduitId,
    status: Option<Status>,
    after: Option<&'a str>,
}

impl<'a> GetConduitShards<'a> {
    pub fn new(client: &'a Client, conduit_id: &'a ConduitId) -> Self {
        Self {
            client,
            conduit_id,
            status: None,
            after: None,
        }
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<GetConduitShardsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS, SHARDS]);

        url.query_pairs_mut()
            .append_pair(CONDUIT_ID, self.conduit_id);
        if let Some(val) = self.status {
            url.query_pairs_mut().append_pair(STATUS, val.as_ref());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateConduitShards<'a> {
    #[serde(skip)]
    client: &'a Client,
    conduit_id: ConduitId,
    shards: Vec<ShardUpdate>,
}

impl<'a> UpdateConduitShards<'a> {
    pub fn new(api: &'a Client, conduit_id: ConduitId) -> Self {
        Self {
            client: api,
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

    pub async fn send(self) -> Result<UpdateConduitShardsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS, SHARDS]);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
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
