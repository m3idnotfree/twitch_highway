use std::fmt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, CustomRewardId, Images, RedemptionId, RewardId, UserId};

#[derive(Debug, Deserialize)]
pub struct CustomRewards {
    pub broadcaster_id: BroadcasterId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_login: Option<String>,
    pub broadcaster_name: String,
    pub id: CustomRewardId,
    pub title: String,
    pub prompt: String,
    pub cost: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Images>,
    pub default_image: Images,
    pub background_color: String,
    pub is_enabled: bool,
    pub is_user_input_required: bool,
    pub max_per_stream_setting: MaxPerStreamSetting,
    pub max_per_user_per_stream_setting: MaxPerStreamSetting,
    pub global_cooldown_setting: MaxPerStreamSetting,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub should_redemptions_skip_request_queue: bool,
    pub redemptions_redeemed_current_stream: Option<u64>,
    pub cooldown_expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomRewardsRedemption {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub id: RedemptionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
    pub user_id: UserId,
    pub user_name: String,
    pub user_input: String,
    pub status: RedemptionStatus,
    pub redeemed_at: DateTime<FixedOffset>,
    pub reward: Reward,
}

#[derive(Debug, Deserialize)]
pub struct MaxPerStreamSetting {
    pub is_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_user_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cooldown_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reward {
    pub id: RewardId,
    pub title: String,
    pub prompt: String,
    pub cost: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RedemptionStatus {
    CANCELED,
    FULFILLED,
    UNFULFILLED,
}

impl RedemptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CANCELED => "CANCELED",
            Self::FULFILLED => "FULFILLED",
            Self::UNFULFILLED => "UNFULFILLED",
        }
    }
}

impl fmt::Display for RedemptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for RedemptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::channel_points::types::{
        CustomRewards, CustomRewardsRedemption, MaxPerStreamSetting, RedemptionStatus,
    };

    #[test]
    fn redemption_status_enum() {
        let statuses = vec![
            (RedemptionStatus::CANCELED, "CANCELED"),
            (RedemptionStatus::FULFILLED, "FULFILLED"),
            (RedemptionStatus::UNFULFILLED, "UNFULFILLED"),
        ];

        for (status, expected_str) in statuses {
            assert_eq!(status.as_str(), expected_str);
            assert_eq!(status.as_ref(), expected_str);
            assert_eq!(status.to_string(), expected_str);

            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: RedemptionStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn max_per_stream_setting_optional_fields() {
        let json_with_all = json!({
            "is_enabled": true,
            "max_per_stream": 5,
            "max_per_user_per_stream": 1,
            "global_cooldown_seconds": 300
        });

        let setting: MaxPerStreamSetting = serde_json::from_value(json_with_all).unwrap();
        assert!(setting.is_enabled);
        assert_eq!(setting.max_per_stream, Some(5));
        assert_eq!(setting.max_per_user_per_stream, Some(1));
        assert_eq!(setting.global_cooldown_seconds, Some(300));

        let json_minimal = json!({
            "is_enabled": false
        });

        let setting_minimal: MaxPerStreamSetting = serde_json::from_value(json_minimal).unwrap();
        assert!(!setting_minimal.is_enabled);
        assert!(setting_minimal.max_per_stream.is_none());
        assert!(setting_minimal.max_per_user_per_stream.is_none());
        assert!(setting_minimal.global_cooldown_seconds.is_none());
    }

    #[test]
    fn reward_with_all_optional_fields() {
        let json_data = json!({
            "broadcaster_id": "123456789",
            "broadcaster_login": "testbroadcaster",
            "broadcaster_name": "TestBroadcaster",
            "id": "reward123",
            "title": "Complete Reward",
            "prompt": "A reward with all fields",
            "cost": 2500,
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
            "background_color": "#FF6B6B",
            "is_enabled": false,
            "is_user_input_required": true,
            "max_per_stream_setting": {
                "is_enabled": true,
                "max_per_stream": 5
            },
            "max_per_user_per_stream_setting": {
                "is_enabled": true,
                "max_per_user_per_stream": 2
            },
            "global_cooldown_setting": {
                "is_enabled": true,
                "global_cooldown_seconds": 600
            },
            "is_paused": true,
            "is_in_stock": false,
            "should_redemptions_skip_request_queue": true,
            "redemptions_redeemed_current_stream": 3,
            "cooldown_expires_at": "2023-12-01T17:30:00Z"
        });

        let reward: CustomRewards = serde_json::from_value(json_data).unwrap();

        assert_eq!(reward.title, "Complete Reward");
        assert_eq!(reward.cost, 2500);
        assert_eq!(reward.background_color, "#FF6B6B");
        assert!(!reward.is_enabled);
        assert!(reward.is_user_input_required);
        assert!(reward.is_paused);
        assert!(!reward.is_in_stock);
        assert!(reward.should_redemptions_skip_request_queue);
        assert_eq!(reward.redemptions_redeemed_current_stream, Some(3));
        assert!(reward.cooldown_expires_at.is_some());

        assert!(reward.image.is_some());
        let image = reward.image.unwrap();
        assert!(image.url_2x.contains("custom-reward-images"));

        assert_eq!(reward.max_per_stream_setting.max_per_stream, Some(5));
        assert_eq!(
            reward
                .max_per_user_per_stream_setting
                .max_per_user_per_stream,
            Some(2)
        );
        assert_eq!(
            reward.global_cooldown_setting.global_cooldown_seconds,
            Some(600)
        );
    }

    #[test]
    fn redemption_with_optional() {
        let json_with_login = json!({
            "broadcaster_id": "123456789",
            "broadcaster_login": "testbroadcaster",
            "broadcaster_name": "TestBroadcaster",
            "id": "redemption123",
            "user_login": "testuser",
            "user_id": "user123",
            "user_name": "TestUser",
            "user_input": "Test input",
            "status": "UNFULFILLED",
            "redeemed_at": "2023-12-01T15:30:00Z",
            "reward": {
                "id": "reward123",
                "title": "Test Reward",
                "prompt": "Test prompt",
                "cost": 1000
            }
        });

        let redemption: CustomRewardsRedemption = serde_json::from_value(json_with_login).unwrap();
        assert_eq!(redemption.user_login, Some("testuser".to_string()));

        let json_without_login = json!({
            "broadcaster_id": "123456789",
            "broadcaster_login": "testbroadcaster",
            "broadcaster_name": "TestBroadcaster",
            "id": "redemption123",
            "user_id": "user123",
            "user_name": "TestUser",
            "user_input": "Test input",
            "status": "UNFULFILLED",
            "redeemed_at": "2023-12-01T15:30:00Z",
            "reward": {
                "id": "reward123",
                "title": "Test Reward",
                "prompt": "Test prompt",
                "cost": 1000
            }
        });

        let redemption_no_login: CustomRewardsRedemption =
            serde_json::from_value(json_without_login).unwrap();
        assert!(redemption_no_login.user_login.is_none());
    }
}
