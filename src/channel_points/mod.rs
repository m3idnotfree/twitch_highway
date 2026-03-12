mod builder;
mod response;
mod types;

pub use builder::{
    CreateCustomRewardBuilder, GetCustomRewardBuilder, GetCustomRewardRedemptionBuilder,
    UpdateCustomRewardBuilder,
};
pub use response::{CustomRewardsRedemptionResponse, CustomRewardsResponse};
pub use types::{
    CustomReward, CustomRewardsRedemption, MaxPerStreamSetting, RedemptionStatus, Sort,
};

use types::UpdateRedemptionStatusBody;

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID, REDEMPTIONS, REWARD_ID},
        BroadcasterId, RedemptionId, RewardId,
    },
    Client, Error,
};

pub trait ChannelPointsAPI {
    /// Creates a Custom Reward in the broadcaster’s channel
    ///
    /// # Returns
    ///
    /// Returns a [`CreateCustomRewardBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::ChannelPointsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_custom_rewards(&BroadcasterId::from("1234"), "title", 64)
    ///     .prompt("prompt")
    ///     .background_color("blue")
    ///     .is_enabled(false)
    ///     .is_user_input_required(true)
    ///     .is_max_per_stream_enabled(true)
    ///     .max_per_stream(5)
    ///     .is_max_per_stream_enabled(true)
    ///     .max_per_user_per_stream(5)
    ///     .is_global_cooldown_enabled(true)
    ///     .global_cooldown_seconds(50)
    ///     .should_redemptions_skip_request_queue(true)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-custom-rewards>
    fn create_custom_rewards<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        cost: u64,
    ) -> CreateCustomRewardBuilder<'a>;

    /// Deletes a custom reward that the broadcaster created
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that created the custom reward.
    /// * `reward_id` - The ID of the custom reward to delete.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::ChannelPointsAPI,
    ///     types::{BroadcasterId, RewardId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .delete_custom_reward(
    ///         &BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-custom-reward>
    fn delete_custom_reward(
        &self,
        broadcaster_id: &BroadcasterId,
        reward_id: &RewardId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets a list of custom rewards that the specified broadcaster created
    ///
    /// # Returns
    ///
    /// Returns a [`GetCustomRewardBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::ChannelPointsAPI,
    ///     types::{BroadcasterId, RewardId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_custom_reward(&BroadcasterId::from("1234"))
    ///     .custom_reward_ids(&[RewardId::from("5678")])
    ///     .only_manageable_rewards(true)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:redemptions or channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward>
    fn get_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetCustomRewardBuilder<'a>;

    /// Gets a list of redemptions for the specified custom reward
    ///
    /// # Returns
    ///
    /// Returns a [`GetCustomRewardRedemptionBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::{ChannelPointsAPI, RedemptionStatus, Sort},
    ///     types::{BroadcasterId, RewardId, RedemptionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_custom_reward_redemption(
    ///         &BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"))
    ///     .status(RedemptionStatus::CANCELED)
    ///     .sort(Sort::NEWEST)
    ///     .ids(&[RedemptionId::from("6789")])
    ///     .first(5)
    ///     .after("eyJiI...")
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:redemptions or channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-custom-reward-redemption>
    fn get_custom_reward_redemption<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> GetCustomRewardRedemptionBuilder<'a>;

    /// Updates a custom reward
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateCustomRewardBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::ChannelPointsAPI,
    ///     types::{BroadcasterId, RewardId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_custom_reward(
    ///         &BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"))
    ///     .title("title")
    ///     .cost(60)
    ///     .prompt("prompt")
    ///     .background_color("blue")
    ///     .is_enabled(false)
    ///     .is_user_input_required(true)
    ///     .is_max_per_stream_enabled(true)
    ///     .max_per_stream(5)
    ///     .max_per_user_per_stream(5)
    ///     .is_global_cooldown_enabled(true)
    ///     .global_cooldown_seconds(50)
    ///     .is_paused(false)
    ///     .should_redemptions_skip_request_queue(true)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-custom-reward>
    fn update_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> UpdateCustomRewardBuilder<'a>;

    /// Updates a redemption’s status
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s updating the redemption.
    /// * `reward_id` - The ID that identifies the reward that’s been redeemed.
    /// * `redemption_ids` - A list of IDs that identify the redemptions to update.
    /// * `status` -
    ///
    /// # Returns
    ///
    /// Returns a [`CustomRewardsRedemptionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channel_points::{ChannelPointsAPI, RedemptionStatus},
    ///     types::{BroadcasterId, RedemptionId, RewardId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_redemption_status(&BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"),
    ///         &[RedemptionId::from("78901")],
    ///         RedemptionStatus::CANCELED)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:redemptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-redemption-status>
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
    ) -> CreateCustomRewardBuilder<'a> {
        CreateCustomRewardBuilder::new(self, broadcaster_id, title, cost)
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

    fn get_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetCustomRewardBuilder<'a> {
        GetCustomRewardBuilder::new(self, broadcaster_id)
    }

    fn get_custom_reward_redemption<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> GetCustomRewardRedemptionBuilder<'a> {
        GetCustomRewardRedemptionBuilder::new(self, broadcaster_id, reward_id)
    }

    fn update_custom_reward<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> UpdateCustomRewardBuilder<'a> {
        UpdateCustomRewardBuilder::new(self, broadcaster_id, reward_id)
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
