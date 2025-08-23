pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use response::HypeTrainResponse;

use crate::{
    hype_train::response::HypeTrainStatusResponse,
    request::EndpointType,
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
};

endpoints! {
    HypeTrainAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
        fn get_hype_train_events(
            &self,
            broadcaster_id: &BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> HypeTrainResponse {
            endpoint_type: EndpointType::GetHypeTrainEvents,
            method: Method::GET,
            path: ["hypetrain", "events"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }

        // <https://dev.twitch.tv/docs/api/reference/#get-hype-train-status>
        fn get_hype_train_status(
            &self,
            broadcaster_id: &BroadcasterId
        ) -> HypeTrainStatusResponse {
            endpoint_type: EndpointType::GetHypeTrainStatus,
            method: Method::GET,
            path: ["hypetrain","status"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hype_train::HypeTrainAPI,
        types::{BroadcasterId, PaginationQuery},
    };

    api_test!(
        get_hype_train_events,
        [
            &BroadcasterId::from("270954519"),
            Some(PaginationQuery::new().first(1))
        ]
    );
    api_test!(get_hype_train_status, [&BroadcasterId::from("123")]);
}
