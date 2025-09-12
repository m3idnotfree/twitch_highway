use serde::{Deserialize, Serialize};

use crate::types::ConduitId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conduit {
    id: ConduitId,
    shard_count: u64,
}
