use crate::{
    base::TwitchAPIBase,
    request::RequestBody,
    types::{
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID},
        BroadcasterId, Id, PaginationQuery,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use request::{
    CustomRewardRedemptionQuery, CustomRewardsBody, CustomRewardsRequiredBody,
    RedemptionStatusQuery, UpdateCustomRewardRequest,
};
use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};

pub mod request;
pub mod response;
pub mod types;

pub trait ChannelPointsAPI: TwitchAPIBase {
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
    fn delete_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    fn get_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Vec<Id>>,
        only_manageable_rewards: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsResponse>;
    fn get_custom_reward_redemption(
        &self,
        broadcaster_id: BroadcasterId,
        reward_id: &str,
        opts: Option<CustomRewardRedemptionQuery>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsRedemptionResponse>;
    fn update_custom_reward(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        opts: Option<UpdateCustomRewardRequest>,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest, CustomRewardsResponse>;
    fn update_redemption_status<L: IntoIterator<Item = Id>>(
        &self,
        broadcaster_id: BroadcasterId,
        id: L,
        reward_id: &str,
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
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

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
        id: Option<Vec<Id>>,
        only_manageable_rewards: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(id.map(|x| x.into_iter().map(|x| (ID, x))))
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
        reward_id: &str,
        opts: Option<CustomRewardRedemptionQuery>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CustomRewardsRedemptionResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query("reward_id", reward_id)
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

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
        id: Id,
        opts: Option<UpdateCustomRewardRequest>,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest, CustomRewardsResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

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
    fn update_redemption_status<L: IntoIterator<Item = Id>>(
        &self,
        broadcaster_id: BroadcasterId,
        id: L,
        reward_id: &str,
        status: RedemptionStatusQuery,
    ) -> TwitchAPIRequest<RedemptionStatusQuery, CustomRewardsRedemptionResponse> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query(BROADCASTER_ID, broadcaster_id.as_str())
            .query("reward_id", reward_id)
            .query_extend(id.into_iter().map(|x| (ID, x)));

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
