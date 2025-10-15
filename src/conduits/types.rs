use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{ConduitId, SessionId, ShardId, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conduit {
    id: ConduitId,
    shard_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConduitShard {
    pub id: ShardId,
    pub status: Status,
    pub transport: Transport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
pub enum Transport {
    Webhook {
        callback: String,
    },
    Websocket {
        session_id: SessionId,
        #[serde(skip_serializing_if = "Option::is_none")]
        connected_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disconnected_at: Option<DateTime<FixedOffset>>,
    },
}

#[derive(Serialize)]
pub(crate) struct CreateConduitsBody {
    pub shard_count: u64,
}

#[derive(Serialize)]
pub(crate) struct UpdateConduitsBody<'a> {
    #[serde(rename = "id")]
    pub conduit_id: &'a ConduitId,
    pub shard_count: u64,
}
