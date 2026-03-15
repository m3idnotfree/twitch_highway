mod builder;
mod response;
mod types;

pub use builder::{
    CreateCustomReward, GetCustomReward, GetCustomRewardRedemption, UpdateCustomReward,
};
pub use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};
pub use types::{
    CustomReward, CustomRewardsRedemption, MaxPerStreamSetting, RedemptionStatus, Sort,
};

use types::UpdateRedemptionStatusBody;

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId, RedemptionId, RewardId,
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID, REDEMPTIONS, REWARD_ID},
    },
};

pub trait ChannelPointsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    fn create_custom_rewards<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        cost: u64,
    ) -> CreateCustomReward<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
    fn delete_custom_reward(
        &self,
        broadcaster_id: &BroadcasterId,
        reward_id: &RewardId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
    fn get_custom_reward<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetCustomReward<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
    fn get_custom_reward_redemption<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> GetCustomRewardRedemption<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
    fn update_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> UpdateCustomReward<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
    fn update_redemption_status(
        &self,
        broadcaster_id: &BroadcasterId,
        reward_id: &RewardId,
        redemption_ids: &[RedemptionId],
        status: RedemptionStatus,
    ) -> impl Future<Output = Result<CustomRewardsRedemptionResponse, Error>> + Send;
}

impl ChannelPointsAPI for Client {
    fn create_custom_rewards<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        cost: u64,
    ) -> CreateCustomReward<'a> {
        CreateCustomReward::new(self, broadcaster_id, title, cost)
    }

    async fn delete_custom_reward(
        &self,
        broadcaster_id: &BroadcasterId,
        reward_id: &RewardId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(ID, reward_id);

        self.no_content(self.http_client().delete(url)).await
    }

    fn get_custom_reward<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetCustomReward<'a> {
        GetCustomReward::new(self, broadcaster_id)
    }

    fn get_custom_reward_redemption<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> GetCustomRewardRedemption<'a> {
        GetCustomRewardRedemption::new(self, broadcaster_id, reward_id)
    }

    fn update_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> UpdateCustomReward<'a> {
        UpdateCustomReward::new(self, broadcaster_id, reward_id)
    }

    async fn update_redemption_status(
        &self,
        broadcaster_id: &BroadcasterId,
        reward_id: &RewardId,
        redemption_ids: &[RedemptionId],
        status: RedemptionStatus,
    ) -> Result<CustomRewardsRedemptionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS, REDEMPTIONS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(REWARD_ID, reward_id)
            .extend_pairs(redemption_ids.iter().map(|id| (ID, id)));

        let req = self
            .http_client()
            .patch(url)
            .json(&UpdateRedemptionStatusBody { status });
        self.json(req).await
    }
}
