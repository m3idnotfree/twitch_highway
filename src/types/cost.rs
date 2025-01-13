use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    pub amount: u64,
    #[serde(rename = "type")]
    pub kind: CostType,
}

impl Cost {
    pub fn new(amount: u64, kind: CostType) -> Self {
        Self { amount, kind }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CostType {
    Bits,
}
