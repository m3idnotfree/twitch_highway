use serde::{Deserialize, Serialize};

use crate::conduits::types::Conduit;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitResponse {
    data: Vec<Conduit>,
}
