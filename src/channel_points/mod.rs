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
            broadcaster_id: &BroadcasterId,
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
        fn delete_custom_reward(
            &self,
            broadcaster_id: &BroadcasterId,
            custom_reward_id: &CustomRewardId,
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
        fn get_custom_reward(
            &self,
            broadcaster_id: &BroadcasterId,
            custom_reward_ids: Option<&[CustomRewardId]>,
            only_manageable_rewards: Option<bool>,
        ) -> CustomRewardsResponse {
            endpoint_type: EndpointType::GetCustomReward,
            method: Method::GET,
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
            broadcaster_id: &BroadcasterId,
            custom_reward_id: &CustomRewardId,
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
            broadcaster_id: &BroadcasterId,
            reward_id: &RewardId,
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
                CustomRewardRedemptionQuery, RedemptionStatusQuery, UpdateCustomRewardRequest,
            },
            types::RedemptionStatus,
            ChannelPointsAPI,
        },
        types::{BroadcasterId, CustomRewardId, RedemptionId, RewardId},
    };
    api_test!(
        create_custom_rewards,
        [
            &BroadcasterId::from("274637212"),
            "game analysis 1v1",
            50000,
            None
        ]
    );
    api_test!(
        delete_custom_reward,
        [
            &BroadcasterId::from("274637212"),
            &CustomRewardId::from("b045196d-9ce7-4a27-a9b9-279ed341ab28"),
        ]
    );
    api_test!(
        get_custom_reward,
        [&BroadcasterId::from("274637212"), None, None]
    );
    api_test!(
        get_custom_reward_redemption,
        [
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            Some(CustomRewardRedemptionQuery::new().status(RedemptionStatus::CANCELED)),
            None
        ]
    );
    api_test!(
        update_custom_reward,
        [
            &BroadcasterId::from("274637212"),
            &CustomRewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            Some(UpdateCustomRewardRequest::new().is_enabled(false)),
        ]
    );
    api_test!(
        update_redemption_status,
        [
            &[RedemptionId::from("17fa2df1-ad76-4804-bfa5-a40ef63efe63")],
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            RedemptionStatusQuery::new(RedemptionStatus::CANCELED),
        ]
    );

    api_test!(extra
        get_custom_reward,
        get_custom_reward2,
        [&BroadcasterId::from("274637212"), None, Some(true)]
    );
    api_test!(extra
        get_custom_reward,
        get_custom_reward3,
        [&BroadcasterId::from("274637212"), Some(&[CustomRewardId::from("2af127c-7326-4483-a52b-b0da0be61c01")]), None]
    );
    api_test!(extra
        get_custom_reward_redemption,
        get_custom_reward_redemption2,
        [
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            None,
            None
        ]
    );
    api_test!(extra
        update_custom_reward,
        update_custom_reward2,
        [
            &BroadcasterId::from("274637212"),
            &CustomRewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            Some(UpdateCustomRewardRequest::new().title("game analysis 2v2")),
        ]
    );
}
