use serde::{Deserialize, Serialize};

use crate::conduits::types::Conduit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConduitResponse {
    data: Vec<Conduit>,
}
