mod request;
mod response;
mod types;

pub use request::StartCommercialBody;
pub use response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};
pub use types::{AdSchedule, SnoozeNextAd, StartCommercial};

use crate::types::{
    constants::{BROADCASTER_ID, CHANNELS},
    BroadcasterId,
};

endpoints! {
    AdsAPI {
        /// Starts a commercial break on the specified channel
        ///
        /// # Arguments
        /// * `broadcaster_id` - The ID of the partner or affiliate broadcaster.
        /// * `length` - The length of the commercial to run, maximum 180.
        ///
        /// # Returns
        ///
        /// Returns a [`StartCommercialResponse`]
        ///
        /// # Example
        ///
        /// ```rust
        /// # use twitch_highway::TwitchAPI;
        /// use twitch_highway::{
        ///     ads::AdsAPI,
        ///     types::BroadcasterId,
        /// };
        ///
        /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
        /// let broadcaster_id = BroadcasterId::from("1234");
        /// let response = api
        ///     .start_commercial(&broadcaster_id, 60)
        ///     .json()
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
            length: u64,
        ) -> StartCommercialResponse {
            endpoint_type: StartCommercial,
            method: POST,
            path: [CHANNELS, "commercial"],
            headers: [json],
            body: StartCommercialBody::new(broadcaster_id, length).into_json()
        }

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
        /// # use twitch_highway::TwitchAPI;
        /// use twitch_highway::{
        ///     ads::AdsAPI,
        ///     types::BroadcasterId,
        /// };
        ///
        /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
        /// let broadcaster_id = BroadcasterId::from("1234");
        /// let response = api
        ///     .get_ad_schedule(&broadcaster_id)
        ///     .json()
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
        ) -> AdScheduleResponse {
            endpoint_type: GetAdSchedule,
            method: GET,
            path: [CHANNELS, "ads"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

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
        /// # use twitch_highway::TwitchAPI;
        /// use twitch_highway::{
        ///     ads::AdsAPI,
        ///     types::BroadcasterId,
        /// };
        ///
        /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
        /// let broadcaster_id = BroadcasterId::from("1234");
        /// let response = api
        ///     .snooze_next_ad(&broadcaster_id)
        ///     .json()
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
        ) -> SnoozeNextAdResponse {
            endpoint_type: SnoozeNextAd,
            method: POST,
            path: [CHANNELS, "ads", "schedule", "snooze"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}
