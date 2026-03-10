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

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, ID, REDEMPTIONS, REWARD_ID},
        BroadcasterId, RedemptionId, RewardId,
    },
    Client,
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
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
    ///     .json()
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
    /// # Returns
    ///
    /// Returns a [`NoContent`]
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .delete_custom_reward(
    ///         &BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"))
    ///     .json()
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
    ) -> TwitchAPIRequest<NoContent>;

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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_custom_reward(&BroadcasterId::from("1234"))
    ///     .custom_reward_ids(&[RewardId::from("5678")])
    ///     .only_manageable_rewards(true)
    ///     .json()
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_custom_reward_redemption(
    ///         &BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"))
    ///     .status(RedemptionStatus::CANCELED)
    ///     .sort(Sort::NEWEST)
    ///     .ids(&[RedemptionId::from("6789")])
    ///     .first(5)
    ///     .after("eyJiI...")
    ///     .json()
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
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
    ///     .json()
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_redemption_status(&BroadcasterId::from("1234"),
    ///         &RewardId::from("5678"),
    ///         &[RedemptionId::from("78901")],
    ///         RedemptionStatus::CANCELED)
    ///     .json()
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
    ) -> TwitchAPIRequest<CustomRewardsRedemptionResponse>;
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
    simple_endpoint!(fn delete_custom_reward(
        broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
        reward_id: &RewardId [key = ID]
    ) -> NoContent;
        endpoint: DeleteCustomReward,
        method: DELETE,
        path: [CHANNEL_POINTS, CUSTOM_REWARDS]
    );
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
    simple_endpoint!(
        fn update_redemption_status(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
            reward_id: &RewardId [key = REWARD_ID],
            redemption_ids: &[RedemptionId] [key = ID, convert = extend],
            status: RedemptionStatus [skip]
        ) -> CustomRewardsRedemptionResponse;
            endpoint: UpdateRedemptionStatus,
            method: PATCH,
            path: [CHANNEL_POINTS, CUSTOM_REWARDS, REDEMPTIONS],
            headers: [json],
            body: {
                serde_json::to_string(&UpdateRedemptionStatusBody {status}).ok()
            }
    );
}
