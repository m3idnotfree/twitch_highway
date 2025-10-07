use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    pub decimal_places: u64,
    pub currency: String,
}

#[cfg(test)]
mod tests {
    use crate::types::Amount;

    #[test]
    fn amount() {
        let amount = Amount {
            value: 2550,
            decimal_places: 2,
            currency: "USD".to_string(),
        };

        assert_eq!(amount.value, 2550);
        assert_eq!(amount.decimal_places, 2);
        assert_eq!(amount.currency, "USD");

        let serialized = serde_json::to_string(&amount).unwrap();
        let expected = json!({
            "value": 2550,
            "decimal_places": 2,
            "currency": "USD"
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            expected
        );

        let deserialized: Amount = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.value, amount.value);
        assert_eq!(deserialized.decimal_places, amount.decimal_places);
        assert_eq!(deserialized.currency, amount.currency);
    }
}
