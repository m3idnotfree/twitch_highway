use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{ConduitId, SessionId};

use super::serde_helpers::deserialize_optional_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConduitShardDisabled {
    pub conduit_id: ConduitId,
    pub shard_id: String,
    pub status: String,
    pub transport: Transport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transport {
    pub method: TransportMethod,
    pub callback: Option<String>,
    pub session_id: Option<SessionId>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub connected_at: Option<DateTime<FixedOffset>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub disconnected_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Websocket,
    Webhook,
}
