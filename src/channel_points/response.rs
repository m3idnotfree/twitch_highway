use serde::Deserialize;

use super::types::{CustomRewards, CustomRewardsRedemption};

#[derive(Debug, Deserialize)]
pub struct CustomRewardsResponse {
    pub data: Option<Vec<CustomRewards>>,
}

#[derive(Debug, Deserialize)]
pub struct CustomRewardsRedemptionResponse {
    pub data: Vec<CustomRewardsRedemption>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::channel_points::response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};

    #[test]
    fn custom_rewards_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "reward123",
                    "title": "Test Reward",
                    "prompt": "This is a test reward for testing",
                    "cost": 1000,
                    "image": {
                        "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/1x.png",
                        "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/2x.png",
                        "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/4x.png"
                    },
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/default-reward-images/1x.png",
                        "url_2x": "https://static-cdn.jtvnw.net/default-reward-images/2x.png",
                        "url_4x": "https://static-cdn.jtvnw.net/default-reward-images/4x.png"
                    },
                    "background_color": "#9147FF",
                    "is_enabled": true,
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": true,
                        "max_per_stream": 10
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": true,
                        "max_per_user_per_stream": 1
                    },
                    "global_cooldown_setting": {
                        "is_enabled": true,
                        "global_cooldown_seconds": 300
                    },
                    "is_paused": false,
                    "is_in_stock": true,
                    "should_redemptions_skip_request_queue": false,
                    "redemptions_redeemed_current_stream": 5,
                    "cooldown_expires_at": "2023-12-01T16:00:00Z"
                }
            ]
        });

        let response: CustomRewardsResponse = serde_json::from_value(json_data).unwrap();

        assert!(response.data.is_some());
        let rewards = response.data.unwrap();
        assert_eq!(rewards.len(), 1);

        let reward = &rewards[0];
        assert_eq!(reward.broadcaster_id.as_str(), "123456789");
        assert_eq!(reward.broadcaster_name, "TestBroadcaster");
        assert_eq!(reward.id.as_str(), "reward123");
        assert_eq!(reward.title, "Test Reward");
        assert_eq!(reward.cost, 1000);
        assert_eq!(reward.background_color, "#9147FF");
        assert!(reward.is_enabled);
        assert!(!reward.is_user_input_required);
        assert!(!reward.is_paused);
        assert!(reward.is_in_stock);

        assert!(reward.max_per_stream_setting.is_enabled);
        assert_eq!(reward.max_per_stream_setting.max_per_stream, Some(10));

        assert!(reward.global_cooldown_setting.is_enabled);
        assert_eq!(
            reward.global_cooldown_setting.global_cooldown_seconds,
            Some(300)
        );

        assert!(reward.image.is_some());
        let image = reward.image.as_ref().unwrap();
        assert!(image.url_1x.contains("1x.png"));

        assert_eq!(reward.redemptions_redeemed_current_stream, Some(5));
        assert_eq!(
            reward.cooldown_expires_at,
            Some("2023-12-01T16:00:00Z".to_string())
        );
    }

    #[test]
    fn custom_rewards_redemption_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "redemption123",
                    "user_login": "testuser",
                    "user_id": "user123",
                    "user_name": "TestUser",
                    "user_input": "This is my redemption request",
                    "status": "UNFULFILLED",
                    "redeemed_at": "2023-12-01T15:30:00Z",
                    "reward": {
                        "id": "reward123",
                        "title": "Test Reward",
                        "prompt": "Test reward prompt",
                        "cost": 1000
                    }
                }
            ]
        });

        let response: CustomRewardsRedemptionResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let redemption = &response.data[0];
        assert_eq!(redemption.broadcaster_id.as_str(), "123456789");
        assert_eq!(redemption.id.as_str(), "redemption123");
        assert_eq!(redemption.user_id.as_str(), "user123");
        assert_eq!(redemption.user_input, "This is my redemption request");

        assert_eq!(redemption.reward.id.as_str(), "reward123");
        assert_eq!(redemption.reward.title, "Test Reward");
        assert_eq!(redemption.reward.cost, 1000);
    }

    #[test]
    fn custom_rewards_response_with_null_data() {
        let json_data = json!({
            "data": null
        });

        let response: CustomRewardsResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_none());
    }

    #[test]
    fn custom_rewards_response_empty_array() {
        let json_data = json!({
            "data": []
        });

        let response: CustomRewardsResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.data.is_some());
        assert_eq!(response.data.unwrap().len(), 0);
    }
}
