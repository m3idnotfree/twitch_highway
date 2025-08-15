use serde::Serialize;

use crate::types::RedemptionId;

use super::types::RedemptionStatus;

define_request!(
    #[derive(Serialize)]
    CustomRewardsRequiredBody<'a> {
        req: {
            title: &'a str,
            cost: u64
        };
    }
);

define_request!(
    #[derive(Serialize)]
    CustomRewardsBody<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            cost: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_user_input_required: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_stream_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_stream: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_user_per_stream_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_user_per_stream: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_global_cooldown_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            global_cooldown_seconds: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            should_redemptions_skip_request_queue: bool
        };
    }
);

define_request!(
    #[derive(Serialize)]
    RedemptionStatusQuery {
        req: {
            status: RedemptionStatus
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    CustomRewardRedemptionQuery<'a> {
        opts: {
            sort: &'a str,
            status: RedemptionStatus,
            id: RedemptionId,
        };
        into_query
    }
);

define_request!(
    #[derive(Serialize, Default)]
    UpdateCustomRewardRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            cost: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_user_input_required: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_stream_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_stream: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_user_per_stream_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_user_per_stream: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_global_cooldown_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            global_cooldown_seconds: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_paused: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            should_redemptions_skip_request_queue: bool
        };
        into_json
    }
);
#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        channel_points::{
            request::{
                CustomRewardRedemptionQuery, CustomRewardsBody, CustomRewardsRequiredBody,
                RedemptionStatusQuery, UpdateCustomRewardRequest,
            },
            types::RedemptionStatus,
        },
        types::RedemptionId,
    };

    #[test]
    fn custom_rewards_required_body_serialization() {
        let body = CustomRewardsRequiredBody::new("Test Reward", 1000);

        let json = serde_json::to_string(&body).unwrap();
        let expected = json!({
            "title": "Test Reward",
            "cost": 1000
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn custom_rewards_body_serialization() {
        let body = CustomRewardsBody::new()
            .title("Updated Reward")
            .prompt("This is a test reward")
            .background_color("#9147FF")
            .cost(500)
            .is_enabled(true)
            .is_user_input_required(false)
            .is_max_per_stream_enabled(true)
            .max_per_stream(10)
            .is_max_per_user_per_stream_enabled(true)
            .max_per_user_per_stream(1)
            .is_global_cooldown_enabled(true)
            .global_cooldown_seconds(300)
            .should_redemptions_skip_request_queue(false);

        let serialized = serde_json::to_string(&body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        assert_eq!(parsed["title"], "Updated Reward");
        assert_eq!(parsed["prompt"], "This is a test reward");
        assert_eq!(parsed["background_color"], "#9147FF");
        assert_eq!(parsed["cost"], 500);
        assert_eq!(parsed["is_enabled"], true);
        assert_eq!(parsed["is_user_input_required"], false);
        assert_eq!(parsed["max_per_stream"], 10);
        assert_eq!(parsed["global_cooldown_seconds"], 300);
    }

    #[test]
    fn update_custom_reward_request_serialization() {
        let request = UpdateCustomRewardRequest::new()
            .title("Updated Title")
            .prompt("Updated prompt")
            .cost(750)
            .is_enabled(false)
            .is_paused(true);

        let json = request.into_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["title"], "Updated Title");
        assert_eq!(parsed["prompt"], "Updated prompt");
        assert_eq!(parsed["cost"], 750);
        assert_eq!(parsed["is_enabled"], false);
        assert_eq!(parsed["is_paused"], true);
    }

    #[test]
    fn redemption_status_query_serialization() {
        let query = RedemptionStatusQuery::new(RedemptionStatus::FULFILLED);

        let json = query.into_json().unwrap();
        let expected = json!({
            "status": "FULFILLED"
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn custom_reward_redemption_query() {
        let query = CustomRewardRedemptionQuery::new()
            .sort("newest")
            .status(RedemptionStatus::UNFULFILLED)
            .id(RedemptionId::new("redemption123"));

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        query.into_query(&mut query_pairs);
        drop(query_pairs);

        let query_str = url.query().unwrap();
        assert!(query_str.contains("sort=newest"));
        assert!(query_str.contains("status=UNFULFILLED"));
        assert!(query_str.contains("id=redemption123"));
    }
}
