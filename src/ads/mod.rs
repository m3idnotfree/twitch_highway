mod response;
mod types;

pub use response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};
pub use types::{AdSchedule, SnoozeNextAd, StartCommercial};

use types::StartCommercialBody;

use std::future::Future;

use crate::{
    types::{
        constants::{ADS, BROADCASTER_ID, CHANNELS, COMMERCIAL, SCHEDULE, SNOOZE},
        BroadcasterId,
    },
    Client, Error,
};

pub trait AdsAPI {
    /// Starts a commercial break on the specified channel
    ///
    /// # Arguments
    /// * `broadcaster_id` - The ID of the partner or affiliate broadcaster.
    /// * `length` - The length of the commercial to run, in seconds maximum 180.
    ///
    /// # Returns
    ///
    /// Returns a [`StartCommercialResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     ads::AdsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let response = api
    ///     .start_commercial(&broadcaster_id, 60)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:edit:commercial`
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
    fn start_commercial(
        &self,
        broadcaster_id: &BroadcasterId,
        length: u8,
    ) -> impl Future<Output = Result<StartCommercialResponse, Error>> + Send;

    /// Gets the broadcaster's ad schedule and details about scheduled ads
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster. must match the user ID in the
    ///   auth token.
    ///
    /// # Returns
    ///
    /// Returns a [`AdScheduleResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     ads::AdsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let response = api
    ///     .get_ad_schedule(&broadcaster_id)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:ads`
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    fn get_ad_schedule(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<AdScheduleResponse, Error>> + Send;

    /// Snoozes the next scheduled ad for the broadcaster
    ///
    /// # Arguments
    /// * `broadcaster_id` - The ID of the broadcaster. must match the user ID in the
    ///   auth token.
    ///
    /// # Returns
    ///
    /// Returns a [`SnoozeNextAdResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     ads::AdsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let response = api
    ///     .snooze_next_ad(&broadcaster_id)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:ads`
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    fn snooze_next_ad(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<SnoozeNextAdResponse, Error>> + Send;
}

impl AdsAPI for Client {
    async fn start_commercial(
        &self,
        broadcaster_id: &BroadcasterId,
        length: u8,
    ) -> Result<StartCommercialResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, COMMERCIAL]);

        let req = self.http_client().post(url).json(&StartCommercialBody {
            broadcaster_id,
            length,
        });
        self.json(req).await
    }

    async fn get_ad_schedule(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<AdScheduleResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, ADS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn snooze_next_ad(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<SnoozeNextAdResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, ADS, SCHEDULE, SNOOZE]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().post(url)).await
    }
}
