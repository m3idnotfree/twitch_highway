use asknothingx2_util::api::Method;
use request::{
    CustomRewardRedemptionQuery, CustomRewardsBody, CustomRewardsRequiredBody,
    RedemptionStatusQuery, UpdateCustomRewardRequest,
};
use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};

use crate::{
    request::RequestBody,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID, REWARD_ID},
        BroadcasterId, CustomRewardId, PaginationQuery, RedemptionId, RewardId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "channel-points")))]
pub trait ChannelPointsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    fn create_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        cost: u64,
        opts: Option<CustomRewardsBody>,
    ) -> TwitchAPIRequest<
        RequestBody<CustomRewardsRequiredBody, CustomRewardsBody>,
        CustomRewardsResponse,
    >;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
    fn delete_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        custom_reward_id: CustomRewardId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
    fn get_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        custom_reward_ids: Option<Vec<CustomRewardId>>,
        only_manageable_rewards: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
    fn get_custom_reward_redemption(
        &self,
        broadcaster_id: BroadcasterId,
        reward_id: RewardId,
        opts: Option<CustomRewardRedemptionQuery>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsRedemptionResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
    fn update_custom_reward(
        &self,
        broadcaster_id: BroadcasterId,
        custom_reward_id: CustomRewardId,
        opts: Option<UpdateCustomRewardRequest>,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest, CustomRewardsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
    fn update_redemption_status(
        &self,
        redemption_ids: &[RedemptionId],
        broadcaster_id: BroadcasterId,
        reward_id: RewardId,
        status: RedemptionStatusQuery,
    ) -> TwitchAPIRequest<RedemptionStatusQuery, CustomRewardsRedemptionResponse>;
}

impl ChannelPointsAPI for TwitchAPI {
    fn create_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        cost: u64,
        opts: Option<CustomRewardsBody>,
    ) -> TwitchAPIRequest<
        RequestBody<CustomRewardsRequiredBody, CustomRewardsBody>,
        CustomRewardsResponse,
    > {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id);

        let request_body = RequestBody::new(
            CustomRewardsRequiredBody::new(title.to_string(), cost),
            opts,
        );

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateCustomRewards,
            url.build(),
            Method::POST,
            headers.build(),
            request_body,
        )
    }
    fn delete_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        custom_reward_id: CustomRewardId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, custom_reward_id);

        TwitchAPIRequest::new(
            EndpointType::DeleteCustomReward,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        custom_reward_ids: Option<Vec<CustomRewardId>>,
        only_manageable_rewards: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(custom_reward_ids.map(|ids| ids.into_iter().map(|id| (ID, id))))
            .query_opt(
                "only_manageable_rewards",
                only_manageable_rewards.map(|x| x.to_string()),
            );

        TwitchAPIRequest::new(
            EndpointType::GetCustomReward,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_custom_reward_redemption(
        &self,
        broadcaster_id: BroadcasterId,
        reward_id: RewardId,
        opts: Option<CustomRewardRedemptionQuery>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsRedemptionResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(REWARD_ID, reward_id);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetCustomRewardRedemption,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_custom_reward(
        &self,
        broadcaster_id: BroadcasterId,
        reward_id: CustomRewardId,
        opts: Option<UpdateCustomRewardRequest>,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest, CustomRewardsResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, reward_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateCustomReward,
            url.build(),
            Method::PATCH,
            headers.build(),
            opts.unwrap_or_default(),
        )
    }
    fn update_redemption_status(
        &self,
        redemption_ids: &[RedemptionId],
        broadcaster_id: BroadcasterId,
        reward_id: RewardId,
        status: RedemptionStatusQuery,
    ) -> TwitchAPIRequest<RedemptionStatusQuery, CustomRewardsRedemptionResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query(BROADCASTER_ID, broadcaster_id.as_str())
            .query(REWARD_ID, reward_id)
            .query_extend(redemption_ids.iter().map(|id| (ID, id)));

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateRedemptionStatus,
            url.build(),
            Method::PATCH,
            headers.build(),
            status,
        )
    }
}
