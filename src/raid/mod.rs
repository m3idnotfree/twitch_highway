use asknothingx2_util::api::Method;
use response::StartRaidResponse;

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId},
    TwitchAPI,
};

pub mod response;
pub mod types;

endpoints! {
    RaidAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
        fn start_raid(
            &self,
            from_broadcaster_id: &BroadcasterId,
            to_broadcaster_id: &BroadcasterId,
        ) -> StartRaidResponse {
            endpoint_type: EndpointType::Startraid,
            method: Method::POST,
            path: ["raids"],
            query_params: {
                extend([
                    ("from_broadcaster_id", from_broadcaster_id),
                    ("to_broadcaster_id", to_broadcaster_id)
                ])
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
        fn cancel_raid(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> NoContent {
            endpoint_type: EndpointType::Cancelraid,
            method: Method::DELETE,
            path: ["raids"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{raid::RaidAPI, types::BroadcasterId};

    api_test!(
        start_raid,
        [
            &BroadcasterId::from("12345678"),
            &BroadcasterId::from("12345678")
        ]
    );
    api_test!(cancel_raid, [&BroadcasterId::from("12345678")]);
}
