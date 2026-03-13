use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CostType {
    Bits,
}
