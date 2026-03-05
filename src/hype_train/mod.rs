mod builder;
mod response;
mod types;

pub use builder::GetHypeTrainEventsBuilder;
pub use response::{HypeTrainResponse, HypeTrainStatusResponse};
pub use types::{
    AllTimeHigh, Current, HypeTrain, HypeTrainContribution, HypeTrainEvent, HypeTrainStatus,
    HypeTrainType, SharedAllTimeHigh, SharedTrainParticipant,
};

use crate::types::{
    constants::{HYPE_TRAIN, STATUS},
    BroadcasterId,
};
use crate::{request::TwitchAPIRequest, TwitchAPI};

pub trait HypeTrainAPI {
    /// # **REMOVED**
    ///
    /// This endpoint war  **removed by Twitch on 2026-02-05**.
    ///
    /// Use [`get_hype_train_status`](HypeTrainAPI::get_hype_train_status) instead.
    ///
    /// See [announcement](https://discuss.dev.twitch.com/t/legacy-get-hype-train-events-api-and-eventsub-hype-train-v1-subscription-types-deprecation-and-withdrawal-timeline/64299)
    ///
    /// Gets information about the broadcaster’s current or most recent Hype Train event
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s running the Hype Train.
    ///
    /// # Returns
    ///
    /// Returns a [`GetHypeTrainEventsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     hype_train::HypeTrainAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_hype_train_events(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:hype_train`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
    #[deprecated(
        since = "0.3.3",
        note = "Use get_hype_train_status instead. Removed by Twitch on Dec 4, 2025"
    )]
    fn get_hype_train_events<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetHypeTrainEventsBuilder<'a>;

    /// Get the status of a Hype Train for the specified broadcaster
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The User ID of the channel broadcaster.
    ///
    /// # Returns
    ///
    /// Returns a [`HypeTrainStatusResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     hype_train::HypeTrainAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_hype_train_status(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:hype_train`
    ///
    /// API Reference
    ///
    // <https://dev.twitch.tv/docs/api/reference/#get-hype-train-status>
    fn get_hype_train_status(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> TwitchAPIRequest<HypeTrainStatusResponse>;
}

impl HypeTrainAPI for TwitchAPI {
    fn get_hype_train_events<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetHypeTrainEventsBuilder<'a> {
        GetHypeTrainEventsBuilder::new(self, broadcaster_id)
    }
    simple_endpoint!(
        fn get_hype_train_status(
            broadcaster_id: &BroadcasterId
        ) -> HypeTrainStatusResponse;
            endpoint: GetHypeTrainStatus,
            method: GET,
            path: [HYPE_TRAIN, STATUS],
    );
}
