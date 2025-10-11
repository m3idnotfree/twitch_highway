use serde::{Deserialize, Serialize};

use crate::{
    conduits::types::{Conduit, ConduitShard},
    types::ShardId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConduitResponse {
    pub data: Vec<Conduit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConduitShardsResponse {
    pub data: Vec<ConduitShard>,
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConduitShardsResponse {
    pub data: Vec<ConduitShard>,
    pub errors: Vec<UpdateShardError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShardError {
    pub id: ShardId,
    pub message: String,
    pub code: String,
}
