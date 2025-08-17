use asknothingx2_util::api::Method;
use request::{
    CustomRewardRedemptionQuery, CustomRewardsBody, CustomRewardsRequiredBody,
    RedemptionStatusQuery, UpdateCustomRewardRequest,
};
use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};

use crate::{
    request::{EndpointType, NoContent, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID, REWARD_ID},
        BroadcasterId, CustomRewardId, PaginationQuery, RedemptionId, RewardId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    ChannelPointsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
        fn create_custom_rewards(
            &self,
            broadcaster_id: BroadcasterId,
            title: &str,
            cost: u64,
            opts: Option<CustomRewardsBody>,
        ) -> CustomRewardsResponse {
            endpoint_type: EndpointType::CreateCustomRewards,
            method: Method::POST,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: RequestBody::new(CustomRewardsRequiredBody::new(title, cost), opts).into_json()
        }
        /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
        fn delete_custom_rewards(
            &self,
            broadcaster_id: BroadcasterId,
            custom_reward_id: CustomRewardId,
        ) -> NoContent {
            endpoint_type: EndpointType::DeleteCustomReward,
            method: Method::DELETE,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, custom_reward_id)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
        fn get_custom_rewards(
            &self,
            broadcaster_id: BroadcasterId,
            custom_reward_ids: Option<&[CustomRewardId]>,
            only_manageable_rewards: Option<bool>,
        ) -> CustomRewardsResponse {
            endpoint_type: EndpointType::GetCustomReward,
            method: Method::GET,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                extend(custom_reward_ids.unwrap_or(&[]).iter().map(|id| (ID, id))),
                opt("only_manageable_rewards", only_manageable_rewards.map(|x| x.to_string()))
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
        fn get_custom_reward_redemption(
            &self,
            broadcaster_id: BroadcasterId,
            reward_id: RewardId,
            opts: Option<CustomRewardRedemptionQuery>,
            pagination: Option<PaginationQuery>,
        ) -> CustomRewardsRedemptionResponse {
            endpoint_type: EndpointType::GetCustomRewardRedemption,
            method: Method::GET,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(REWARD_ID, reward_id),
                opt_into_query(opts),
                pagination(pagination)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
        fn update_custom_reward(
            &self,
            broadcaster_id: BroadcasterId,
            custom_reward_id: CustomRewardId,
            opts: Option<UpdateCustomRewardRequest>,
        ) -> CustomRewardsResponse {
            endpoint_type: EndpointType::UpdateCustomReward,
            method: Method::PATCH,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, custom_reward_id)
            },
            headers: [json],
            body: opts.and_then(|o| o.into_json())
        }
        /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
        fn update_redemption_status(
            &self,
            redemption_ids: &[RedemptionId],
            broadcaster_id: BroadcasterId,
            reward_id: RewardId,
            status: RedemptionStatusQuery,
        ) -> CustomRewardsRedemptionResponse {
            endpoint_type: EndpointType::UpdateRedemptionStatus,
            method: Method::PATCH,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id.as_str()),
                query(REWARD_ID, reward_id),
                extend(redemption_ids.iter().map(|id| (ID, id)))
            },
            headers: [json],
            body: status.into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        channel_points::{
            request::{
                CustomRewardRedemptionQuery, CustomRewardsBody, RedemptionStatusQuery,
                UpdateCustomRewardRequest,
            },
            types::RedemptionStatus,
            ChannelPointsAPI,
        },
        test_utils::TwitchApiTest,
        types::{BroadcasterId, CustomRewardId, PaginationQuery, RedemptionId, RewardId},
    };

    #[tokio::test]
    pub(crate) async fn create_custom_rewards() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let opts = CustomRewardsBody::new()
            .prompt("This is a new test reward")
            .is_user_input_required(true);

        let response = suite
            .execute("/channel_points/custom_rewards", |api| {
                api.create_custom_rewards(
                    BroadcasterId::new("123456789"),
                    "New Test Reward",
                    1500,
                    Some(opts),
                )
            })
            .json()
            .await
            .unwrap();

        assert!(response.data.is_some());
        let rewards = response.data.unwrap();
        assert_eq!(rewards.len(), 1);
        assert_eq!(rewards[0].title, "New Test Reward");
        assert_eq!(rewards[0].cost, 1500);
    }

    #[tokio::test]
    pub(crate) async fn get_custom_rewards() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let custom_reward_ids = vec![CustomRewardId::new("reward123")];

        let response = suite
            .execute("/channel_points/custom_rewards", |api| {
                api.get_custom_rewards(
                    BroadcasterId::new("123456789"),
                    Some(&custom_reward_ids),
                    Some(true),
                )
            })
            .json()
            .await
            .unwrap();

        assert!(response.data.is_some());
        let rewards = response.data.unwrap();
        assert_eq!(rewards.len(), 1);
        assert_eq!(rewards[0].id.as_str(), "reward123");
        assert_eq!(rewards[0].title, "Existing Reward");
    }

    #[tokio::test]
    pub(crate) async fn get_custom_reward_redemption() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let opts = CustomRewardRedemptionQuery::new().status(RedemptionStatus::UNFULFILLED);
        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/channel_points/custom_rewards/redemptions", |api| {
                api.get_custom_reward_redemption(
                    BroadcasterId::new("123456789"),
                    RewardId::new("reward123"),
                    Some(opts),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        let redemption = &response.data[0];
        assert_eq!(redemption.id.as_str(), "redemption123");
        assert_eq!(redemption.user_input, "Please give me the reward!");
    }

    #[tokio::test]
    pub(crate) async fn update_redemption_status() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let redemption_ids = vec![RedemptionId::new("redemption123")];
        let status = RedemptionStatusQuery::new(RedemptionStatus::FULFILLED);

        let response = suite
            .execute("/channel_points/custom_rewards/redemptions", |api| {
                api.update_redemption_status(
                    &redemption_ids,
                    BroadcasterId::new("123456789"),
                    RewardId::new("reward123"),
                    status,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        let redemption = &response.data[0];
        assert_eq!(redemption.id.as_str(), "redemption123");
    }

    #[tokio::test]
    pub(crate) async fn delete_custom_rewards() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let response = suite
            .execute("/channel_points/custom_rewards", |api| {
                api.delete_custom_rewards(
                    BroadcasterId::new("123456789"),
                    CustomRewardId::new("reward123"),
                )
            })
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    pub(crate) async fn update_custom_reward() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_success().await;

        let update_request = UpdateCustomRewardRequest::new()
            .title("Updated Reward Title")
            .prompt("Updated reward prompt")
            .cost(1500)
            .is_enabled(false)
            .is_paused(true);

        let response = suite
            .execute("/channel_points/custom_rewards", |api| {
                api.update_custom_reward(
                    BroadcasterId::new("123456789"),
                    CustomRewardId::new("reward123"),
                    Some(update_request),
                )
            })
            .json()
            .await
            .unwrap();

        assert!(response.data.is_some());
        let rewards = response.data.unwrap();
        assert_eq!(rewards.len(), 1);
        assert_eq!(rewards[0].title, "Updated Reward Title");
        assert_eq!(rewards[0].cost, 1500);
        assert!(!rewards[0].is_enabled);
        assert!(rewards[0].is_paused);
    }

    #[tokio::test]
    async fn channel_points_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channel_points_failure().await;

        let response = suite
            .execute("/channel_points/custom_rewards", |api| {
                api.create_custom_rewards(
                    BroadcasterId::new("123456789"),
                    "Invalid Reward",
                    0,
                    None,
                )
            })
            .json()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
