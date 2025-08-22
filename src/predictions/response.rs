use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::Prediction;

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionsResponse {
    pub data: Vec<Prediction>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn predictions_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "prediction123",
                    "broadcaster_id": "broadcaster456",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_login": "teststreamer",
                    "title": "Will we reach 1000 followers today?",
                    "winning_outcome_id": null,
                    "outcomes": [
                        {
                            "id": "outcome1",
                            "title": "Yes",
                            "users": 150,
                            "channel_points": 75000,
                            "top_predictors": [
                                {
                                    "user_id": "predictor1",
                                    "user_name": "BigPredictor",
                                    "user_login": "bigpredictor",
                                    "channel_points_used": 10000,
                                    "channel_points_won": 0
                                },
                            ],
                            "color": "BLUE"
                        },
                        {
                            "id": "outcome2",
                            "title": "No",
                            "users": 80,
                            "channel_points": 40000,
                            "top_predictors": null,
                            "color": "PINK"
                        }
                    ],
                    "prediction_window": 1800,
                    "status": "ACTIVE",
                    "created_at": "2024-01-15T10:30:00Z",
                    "ended_at": null,
                    "locked_at": null
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: PredictionsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());

        let prediction = &response.data[0];
        assert_eq!(prediction.id.as_str(), "prediction123");
        assert_eq!(prediction.broadcaster_id.as_str(), "broadcaster456");
        assert_eq!(prediction.title, "Will we reach 1000 followers today?");
        assert_eq!(prediction.outcomes.len(), 2);
        assert_eq!(prediction.prediction_window, 1800);
        assert!(prediction.winning_outcome_id.is_none());

        let first_outcome = &prediction.outcomes[0];
        assert_eq!(first_outcome.title, "Yes");
        assert_eq!(first_outcome.users, 150);
        assert_eq!(first_outcome.channel_points, 75000);
        assert!(first_outcome.top_predictors.is_some());

        let second_outcome = &prediction.outcomes[1];
        assert_eq!(second_outcome.title, "No");
        assert_eq!(second_outcome.users, 80);
        assert_eq!(second_outcome.channel_points, 40000);
        assert!(second_outcome.top_predictors.is_none());
    }

    #[test]
    fn predictions_response_empty_data() {
        let json_data = json!({
            "data": [],
            "pagination": {}
        });

        let response: PredictionsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 0);
        assert!(response.pagination.is_none());
    }
}
