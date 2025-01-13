use serde::{Deserialize, Serialize};

use super::types::Ccls;

#[derive(Debug, Serialize, Deserialize)]
pub struct CclsResponse {
    pub data: Vec<Ccls>,
}
