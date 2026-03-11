mod builder;
mod response;
mod types;

pub use builder::GetHypeTrainEventsBuilder;
pub use response::{HypeTrainResponse, HypeTrainStatusResponse};
pub use types::{
    AllTimeHigh, Current, HypeTrain, HypeTrainContribution, HypeTrainEvent, HypeTrainStatus,
    HypeTrainType, SharedAllTimeHigh, SharedTrainParticipant,
};

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, HYPE_TRAIN, STATUS},
        BroadcasterId,
    },
    Client, Error,
};

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     hype_train::HypeTrainAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     hype_train::HypeTrainAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_hype_train_status(&BroadcasterId::from("1234"))
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
    ) -> impl Future<Output = Result<HypeTrainStatusResponse, Error>> + Send;
}

impl HypeTrainAPI for Client {
    fn get_hype_train_events<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetHypeTrainEventsBuilder<'a> {
        GetHypeTrainEventsBuilder::new(self, broadcaster_id)
    }

    async fn get_hype_train_status(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<HypeTrainStatusResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([HYPE_TRAIN, STATUS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }
}
