use serde::{Deserialize, Serialize};

use crate::types::ConduitId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conduit {
    id: ConduitId,
    shard_count: u64,
}
