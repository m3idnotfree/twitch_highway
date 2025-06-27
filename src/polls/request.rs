use serde::Serialize;

use crate::types::{BroadcasterId, Id};

use super::types::PollStatus;

define_request!(
    #[derive(Serialize)]
    PollsRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            channel_points_voting_enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            channel_points_per_vote: u64,
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    EndPollRequest {
        req: {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PollStatus
        };
        into_request_body
    }
);
