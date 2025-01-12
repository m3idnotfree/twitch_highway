use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, Id, BROADCASTER_ID, ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use request::{
    CreateCustomRewardsRequest, CustomRewardRedemptionRequest, UpdateCustomRewardRequest,
    UpdateRedemptionStatusRequest,
};

pub mod request;
pub mod response;
pub mod types;

const CHANNEL_POINTS: &str = "channel_points";
const CUSTOM_REWARDS: &str = "custom_rewards";

pub trait ChannelPointsAPI: TwitchAPIBase {
    fn create_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        request: CreateCustomRewardsRequest,
    ) -> TwitchAPIRequest<CreateCustomRewardsRequest>;
    fn delete_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Vec<Id>>,
        only_manageable_rewards: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_custom_reward_redemption(
        &self,
        request: CustomRewardRedemptionRequest,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_custom_reward(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        request: UpdateCustomRewardRequest,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest>;
    fn update_redemption_status<L: IntoIterator<Item = Id>>(
        &self,
        broadcaster_id: BroadcasterId,
        id: L,
        reward_id: &str,
        status: UpdateRedemptionStatusRequest,
    ) -> TwitchAPIRequest<UpdateRedemptionStatusRequest>;
}

impl ChannelPointsAPI for TwitchAPI {
    fn create_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        request: CreateCustomRewardsRequest,
    ) -> TwitchAPIRequest<CreateCustomRewardsRequest> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query([(BROADCASTER_ID, broadcaster_id)]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateCustomRewards,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn delete_custom_rewards(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query([(BROADCASTER_ID, broadcaster_id)])
            .query([(ID, id)]);

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
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query([(BROADCASTER_ID, broadcaster_id)])
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
        request: CustomRewardRedemptionRequest,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query_pairs(request);

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
        request: UpdateCustomRewardRequest,
    ) -> TwitchAPIRequest<UpdateCustomRewardRequest> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS])
            .query([(BROADCASTER_ID, broadcaster_id)])
            .query([(ID, id)]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::UpdateCustomReward,
            url.build(),
            Method::PATCH,
            headers.build(),
            request,
        )
    }
    fn update_redemption_status<L: IntoIterator<Item = Id>>(
        &self,
        broadcaster_id: BroadcasterId,
        id: L,
        reward_id: &str,
        status: UpdateRedemptionStatusRequest,
    ) -> TwitchAPIRequest<UpdateRedemptionStatusRequest> {
        let mut url = self.build_url();
        url.path([CHANNEL_POINTS, CUSTOM_REWARDS, "redemptions"])
            .query([(BROADCASTER_ID, broadcaster_id)])
            .query([("reward_id", reward_id)])
            .query(id.into_iter().map(|x| (ID, x)));

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
