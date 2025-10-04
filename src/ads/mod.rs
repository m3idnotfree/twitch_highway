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
        /// # Required Scope
        /// `channel:edit:commercial`
        ///
        /// # Reference
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
        /// * `broadcaster_id` - The ID of the broadcaster. must match the user ID in the
        ///   auth token.
        ///
        /// # Required Scope
        /// `channel:read:ads`
        ///
        /// # Reference
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
        /// # Required Scope
        /// `channel:manage:ads`
        ///
        /// # Reference
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

#[cfg(test)]
mod tests {
    use crate::{ads::AdsAPI, types::BroadcasterId};

    api_test!(start_commercial, [&BroadcasterId::from("141981764"), 60]);
    api_test!(get_ad_schedule, [&BroadcasterId::from("123")]);
    api_test!(snooze_next_ad, [&BroadcasterId::from("123")]);
}
