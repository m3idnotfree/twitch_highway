use serde::Serialize;

use crate::types::{ConduitId, SessionId};

#[derive(Debug, Serialize)]
pub struct UpdateConduitShardsRequest {
    conduit_id: ConduitId,
    shards: Vec<Shard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<SessionId>,
}

impl UpdateConduitShardsRequest {
    pub fn new(conduit_id: ConduitId) -> Self {
        Self {
            conduit_id,
            shards: Vec::new(),
            session_id: None,
        }
    }

    pub fn shard(mut self, shard: Shard) -> Self {
        self.shards.push(shard);
        self
    }

    pub fn extend_shards(mut self, shards: impl IntoIterator<Item = Shard>) -> Self {
        self.shards.extend(shards);
        self
    }

    pub fn into_json(self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct Shard {
    id: String,
    transport: Transport,
}

impl Shard {
    pub fn websocket(id: String) -> Self {
        Self {
            id,
            transport: Transport {
                method: "websocket".to_string(),
                callback: None,
                secret: None,
            },
        }
    }

    pub fn webhook(id: String, callback: Option<String>, secret: Option<String>) -> Self {
        Self {
            id,
            transport: Transport {
                method: "webhook".to_string(),
                callback,
                secret,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Transport {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<String>,
}
