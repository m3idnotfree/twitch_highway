mod response;
mod types;

pub use response::StartRaidResponse;
pub use types::StartRaid;

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, FROM_BROADCASTER_ID, RAIDS, TO_BROADCASTER_ID},
        BroadcasterId,
    },
    TwitchAPI,
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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     raid::RaidAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let from_broadcaster_id = BroadcasterId::from("1234");
    /// let to_broadcaster_id = BroadcasterId::from("5678");
    /// let response = api
    ///     .start_raid(&from_broadcaster_id, &to_broadcaster_id)
    ///     .json()
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
    ) -> TwitchAPIRequest<StartRaidResponse>;

    /// Cancel a pending raid
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     raid::RaidAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .cancel_raid(&BroadcasterId::from("1234"))
    ///     .json()
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
    fn cancel_raid(&self, broadcaster_id: &BroadcasterId) -> TwitchAPIRequest<NoContent>;
}

impl RaidAPI for TwitchAPI {
    simple_endpoint!(
        fn start_raid(
            from_broadcaster_id: &BroadcasterId [key = FROM_BROADCASTER_ID],
            to_broadcaster_id: &BroadcasterId [key = TO_BROADCASTER_ID],
        ) -> StartRaidResponse;
            endpoint: Startraid,
            method: POST,
            path: [RAIDS],
    );
    simple_endpoint!(
        fn cancel_raid(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
        ) -> NoContent ;
            endpoint: Cancelraid,
            method: DELETE,
            path: [RAIDS],
    );
}
