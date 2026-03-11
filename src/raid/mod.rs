mod response;
mod types;

pub use response::StartRaidResponse;
pub use types::StartRaid;

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, FROM_BROADCASTER_ID, RAIDS, TO_BROADCASTER_ID},
        BroadcasterId,
    },
    Client, Error,
};

pub trait RaidAPI {
    /// Raid another channel by sending the broadcaster’s viewers to the targeted channel
    ///
    /// # Arguments
    ///
    /// * `from_broadcaster_id` - The ID of the broadcaster that’s sending the raiding party.
    /// * `to_broadcaster_id` - The ID of the broadcaster to raid.
    ///
    /// # Returns
    ///
    /// Returns a [`StartRaidResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     raid::RaidAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let from_broadcaster_id = BroadcasterId::from("1234");
    /// let to_broadcaster_id = BroadcasterId::from("5678");
    /// let response = api
    ///     .start_raid(&from_broadcaster_id, &to_broadcaster_id)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:raids`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    fn start_raid(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<StartRaidResponse, Error>> + Send;

    /// Cancel a pending raid
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     raid::RaidAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .cancel_raid(&BroadcasterId::from("1234"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:raids`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    fn cancel_raid(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl RaidAPI for Client {
    async fn start_raid(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
    ) -> Result<StartRaidResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RAIDS);

        url.query_pairs_mut()
            .append_pair(FROM_BROADCASTER_ID, from_broadcaster_id)
            .append_pair(TO_BROADCASTER_ID, to_broadcaster_id);

        self.json(self.http_client().post(url)).await
    }

    async fn cancel_raid(&self, broadcaster_id: &BroadcasterId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RAIDS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.no_content(self.http_client().delete(url)).await
    }
}
