mod response;
mod types;

pub use response::{HypeTrainResponse, HypeTrainStatusResponse};
pub use types::{
    AllTimeHigh, ContributionType, Current, HypeTrain, HypeTrainContribution, HypeTrainEvent,
    HypeTrainStatus, HypeTrainType, SharedAllTimeHigh, SharedTrainParticipant, TopContribution,
};

use crate::types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery};

endpoints! {
    HypeTrainAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
        fn get_hype_train_events(
            &self,
            broadcaster_id: &BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> HypeTrainResponse {
            endpoint_type: GetHypeTrainEvents,
            method: GET,
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
            endpoint_type: GetHypeTrainStatus,
            method: GET,
            path: ["hypetrain","status"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
            }
        }
    }
}
