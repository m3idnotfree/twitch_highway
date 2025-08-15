#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::types::{DateRange, ExtensionId, GameId};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionAnalytics {
    pub extension_id: ExtensionId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameAnalytics {
    pub game_id: GameId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::analytics::types::ExtensionAnalytics;

    #[test]
    fn analytics_url_field_capitalization() {
        let json_with_capital_url = json!({
            "extension_id": "ext123",
            "URL": "https://example.com/data.csv",
            "type": "overview_v2",
            "date_range": {
                "started_at": "2023-12-01T00:00:00Z",
                "ended_at": "2023-12-01T23:59:59Z"
            }
        });

        let analytics: ExtensionAnalytics = serde_json::from_value(json_with_capital_url).unwrap();
        assert_eq!(analytics.URL, "https://example.com/data.csv");

        let serialized = serde_json::to_value(&analytics).unwrap();
        assert!(serialized.get("URL").is_some());
        assert!(serialized.get("url").is_none()); // lowercase should not exist
    }
}
