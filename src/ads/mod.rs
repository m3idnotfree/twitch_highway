use asknothingx2_util::api::Method;
use request::StartCommercialBody;
use response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};

use crate::{
    request::EndpointType,
    types::{
        constants::{BROADCASTER_ID, CHANNELS},
        BroadcasterId,
    },
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    AdsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
        fn start_commercial(
            &self,
            broadcaster_id: &BroadcasterId,
            length: u64,
        ) -> StartCommercialResponse {
            endpoint_type: EndpointType::StartCommercial,
            method: Method::POST,
            path: [CHANNELS, "commercial"],
            headers: [json],
            body: StartCommercialBody::new(broadcaster_id, length).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
        fn get_ad_schedule(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> AdScheduleResponse {
            endpoint_type: EndpointType::GetAdSchedule,
            method: Method::GET,
            path: [CHANNELS, "ads"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
        fn snooze_next_ad(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> SnoozeNextAdResponse {
            endpoint_type: EndpointType::SnoozeNextAd,
            method: Method::POST,
            path: [CHANNELS, "ads", "schedule", "snooze"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::BroadcasterId;

    use super::AdsAPI;

    api_test!(start_commercial, [&BroadcasterId::from("141981764"), 60]);
    api_test!(get_ad_schedule, [&BroadcasterId::from("123")]);
    api_test!(snooze_next_ad, [&BroadcasterId::from("123")]);
}
