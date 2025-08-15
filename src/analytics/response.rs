use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{ExtensionAnalytics, GameAnalytics};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionAnalyticsResponse {
    pub data: Vec<ExtensionAnalytics>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameAnalyticsResponse {
    pub data: Vec<GameAnalytics>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::analytics::response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};

    #[test]
    fn extension_analytics_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "extension_id": "ext123456",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension123.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                },
                {
                    "extension_id": "ext789012",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension789.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-11-01T00:00:00Z",
                        "ended_at": "2023-11-30T23:59:59Z"
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: ExtensionAnalyticsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first = &response.data[0];
        assert_eq!(first.extension_id.as_str(), "ext123456");
        assert_eq!(
            first.URL,
            "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension123.csv"
        );
        assert_eq!(first.kind, "overview_v2");
        // assert_eq!(first.date_range.started_at, "2023-12-01T00:00:00Z");
        // assert_eq!(first.date_range.ended_at, "2023-12-01T23:59:59Z");

        let second = &response.data[1];
        assert_eq!(second.extension_id.as_str(), "ext789012");
        // assert_eq!(second.date_range.started_at, "2023-11-01T00:00:00Z");

        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.as_ref().unwrap().cursor, "eyJiI...");
    }

    #[test]
    fn game_analytics_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "game_id": "12345",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/game12345.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                }
            ],
            "pagination": {}
        });

        let response: GameAnalyticsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let game_analytics = &response.data[0];
        assert_eq!(game_analytics.game_id.as_str(), "12345");
        assert_eq!(
            game_analytics.URL,
            "https://twitch-analytics.s3-us-west-2.amazonaws.com/game12345.csv"
        );
        assert_eq!(game_analytics.kind, "overview_v2");

        assert!(response.pagination.is_none());
    }

    #[test]
    fn test_extension_analytics_response_no_pagination() {
        let json_data = json!({
            "data": [
                {
                    "extension_id": "ext123456",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension123.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                }
            ]
        });

        let response: ExtensionAnalyticsResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }
}
