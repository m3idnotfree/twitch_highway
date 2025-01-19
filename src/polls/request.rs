use serde::Serialize;

use crate::types::{BroadcasterId, Id};

use super::types::PollStatus;

request_struct!(
    #[derive(Serialize)]
    PollsRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_voting_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_per_vote: u64,
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    EndPollRequest {
        required {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PollStatus
        }
    };
    impl_body: true
);
