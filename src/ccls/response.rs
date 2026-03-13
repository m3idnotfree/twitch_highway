use serde::{Deserialize, Serialize};

use crate::ccls::types::Ccl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CclsResponse {
    pub data: Vec<Ccl>,
}
