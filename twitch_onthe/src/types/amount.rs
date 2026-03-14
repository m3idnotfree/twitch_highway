use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    pub decimal_places: u64,
    pub currency: String,
}
