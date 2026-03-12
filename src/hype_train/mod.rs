mod response;
mod types;

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
