use asknothingx2_util::api::Method;
use response::GoalsResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId},
    TwitchAPI,
};

pub mod response;
pub mod types;

endpoints! {
    GoalsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
        fn get_creator_goals(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> GoalsResponse {
            endpoint_type: EndpointType::GetCreatorGoals,
            method: Method::GET,
            path: ["goals"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{goals::GoalsAPI, types::BroadcasterId};

    api_test!(get_creator_goals, [&BroadcasterId::new("141981764")]);
}
