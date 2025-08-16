use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::HypeTrain;

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainResponse {
    pub data: Vec<HypeTrain>,
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

    use crate::hype_train::response::HypeTrainResponse;

    #[test]
    fn hype_train_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "hypetrain123",
                    "event_type": "hypetrain.progression",
                    "event_timestamp": "2024-01-15T10:30:00Z",
                    "version": "1.0",
                    "event_data": {
                        "broadcaster_id": "broadcaster456",
                        "cooldown_end_time": "2024-01-15T12:30:00Z",
                        "expires_at": "2024-01-15T11:30:00Z",
                        "goal": 5000,
                        "id": "eventdata456",
                        "last_contribution": {
                            "total": 100,
                            "type": "bits",
                            "user": "lastuser"
                        },
                        "level": 2,
                        "started_at": "2024-01-15T10:00:00Z",
                        "top_contributions": [
                            {
                                "total": 500,
                                "type": "subscription",
                                "user": "topuser1"
                            },
                            {
                                "total": 300,
                                "type": "bits",
                                "user": "topuser2"
                            }
                        ],
                        "total": 2500
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: HypeTrainResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.unwrap().cursor, "eyJiI...");

        let hype_train = &response.data[0];
        assert_eq!(hype_train.id.as_str(), "hypetrain123");
        assert_eq!(hype_train.event_type, "hypetrain.progression");
        assert_eq!(hype_train.version, "1.0");

        let event_data = &hype_train.event_data;
        assert_eq!(event_data.broadcaster_id.as_str(), "broadcaster456");
        assert_eq!(event_data.goal, 5000);
        assert_eq!(event_data.level, 2);
        assert_eq!(event_data.total, 2500);
        assert_eq!(event_data.last_contribution.total, 100);
        assert_eq!(event_data.top_contributions.len(), 2);
    }
}
