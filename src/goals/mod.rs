pub mod response;
pub mod types;

use response::GoalsResponse;

use crate::types::{constants::BROADCASTER_ID, BroadcasterId};

endpoints! {
    GoalsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
        fn get_creator_goals(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> GoalsResponse {
            endpoint_type: GetCreatorGoals,
            method: GET,
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

    api_test!(get_creator_goals, [&BroadcasterId::from("141981764")]);
}
