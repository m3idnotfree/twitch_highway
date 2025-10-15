use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{
    conduits::types::{Conduit, ConduitShard},
    types::{Pagination, ShardId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConduitResponse {
    pub data: Vec<Conduit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConduitShardsResponse {
    pub data: Vec<ConduitShard>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
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
