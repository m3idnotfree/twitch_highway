mod request;
mod response;
mod types;

pub use request::{
    CustomRewardRedemptionQuery, CustomRewardsBody, CustomRewardsRequiredBody,
    RedemptionStatusQuery, UpdateCustomRewardRequest,
};
pub use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};
pub use types::{CustomReward, CustomRewardsRedemption, MaxPerStreamSetting, RedemptionStatus};

use crate::{
    request::{NoContent, RequestBody},
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, PaginationQuery, RedemptionId, RewardId,
    },
};

const CHANNEL_POINTS: &str = "channel_points";
const CUSTOM_REWARDS: &str = "custom_rewards";
const REWARD_ID: &str = "reward_id";

endpoints! {
    ChannelPointsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
        fn create_custom_rewards(
            &self,
            broadcaster_id: &BroadcasterId,
            title: &str,
            cost: u64,
            opts: Option<CustomRewardsBody>,
        ) -> CustomRewardsResponse {
            endpoint_type: CreateCustomRewards,
            method: POST,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: RequestBody::new(CustomRewardsRequiredBody::new(title, cost), opts).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
        fn delete_custom_reward(
            &self,
            broadcaster_id: &BroadcasterId,
            reward_id: &RewardId,
        ) -> NoContent {
            endpoint_type: DeleteCustomReward,
            method: DELETE,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, reward_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
        fn get_custom_reward(
            &self,
            broadcaster_id: &BroadcasterId,
            custom_reward_ids: Option<&[RewardId]>,
            only_manageable_rewards: Option<bool>,
        ) -> CustomRewardsResponse {
            endpoint_type: GetCustomReward,
            method: GET,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_extend(custom_reward_ids.map(|ids| ids.iter().map(|id| (ID, id)))),
                opt("only_manageable_rewards", only_manageable_rewards.map(|x| x.to_string()))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
        fn get_custom_reward_redemption(
            &self,
            broadcaster_id: &BroadcasterId,
            reward_id: &RewardId,
            status: Option<RedemptionStatus>,
            opts: Option<CustomRewardRedemptionQuery>,
            pagination: Option<PaginationQuery>,
        ) -> CustomRewardsRedemptionResponse {
            endpoint_type: GetCustomRewardRedemption,
            method: GET,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(REWARD_ID, reward_id),
                opt("status", status),
                opt_into_query(opts),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
        fn update_custom_reward(
            &self,
            broadcaster_id: &BroadcasterId,
            reward_id: &RewardId,
            update: UpdateCustomRewardRequest,
        ) -> CustomRewardsResponse {
            endpoint_type: UpdateCustomReward,
            method: PATCH,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, reward_id)
            },
            headers: [json],
            body: update.into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
        fn update_redemption_status(
            &self,
            broadcaster_id: &BroadcasterId,
            reward_id: &RewardId,
            redemption_ids: &[RedemptionId],
            status: RedemptionStatus,
        ) -> CustomRewardsRedemptionResponse {
            endpoint_type: UpdateRedemptionStatus,
            method: PATCH,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id.as_str()),
                query(REWARD_ID, reward_id),
                extend(redemption_ids.iter().map(|id| (ID, id)))
            },
            headers: [json],
            body: RedemptionStatusQuery::new(status).into_json()
        }
    }
}
