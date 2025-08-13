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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::{Cost, CostType};

    #[test]
    fn rountrip_serde() {
        let cost = Cost::new(150, CostType::Bits);

        let serialized = serde_json::to_string(&cost).unwrap();
        let expected = json!({
            "amount": 150,
            "type": "bits"
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            expected
        );

        let deserialized: Cost = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.amount, 150);
    }
}
