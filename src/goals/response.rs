use serde::{Deserialize, Serialize};

use super::types::Goal;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalsResponse {
    pub data: Vec<Goal>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::goals::response::GoalsResponse;

    #[test]
    fn goals_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "goal123",
                    "broadcaster_id": "broadcaster456",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_login": "teststreamer",
                    "type": "follower",
                    "description": "Reach 1000 followers!",
                    "current_amount": 850,
                    "target_amount": 1000,
                    "created_at": "2024-01-15T10:30:00Z"
                },
                {
                    "id": "goal456",
                    "broadcaster_id": "broadcaster456",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_login": "teststreamer",
                    "type": "subscription",
                    "description": "100 new subscribers this month",
                    "current_amount": 75,
                    "target_amount": 100,
                    "created_at": "2024-01-01T00:00:00Z"
                }
            ]
        });

        let response: GoalsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_goal = &response.data[0];
        assert_eq!(first_goal.id.as_str(), "goal123");
        assert_eq!(first_goal.broadcaster_id.as_str(), "broadcaster456");
        assert_eq!(first_goal.broadcaster_name, "TestStreamer");
        assert_eq!(first_goal.broadcaster_login, "teststreamer");
        assert_eq!(first_goal.description, "Reach 1000 followers!");
        assert_eq!(first_goal.current_amount, 850);
        assert_eq!(first_goal.target_amount, 1000);

        let second_goal = &response.data[1];
        assert_eq!(second_goal.id.as_str(), "goal456");
        assert_eq!(second_goal.description, "100 new subscribers this month");
        assert_eq!(second_goal.current_amount, 75);
        assert_eq!(second_goal.target_amount, 100);
    }

    #[test]
    fn goals_response_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: GoalsResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
    }
}
