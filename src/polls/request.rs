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
    }
);

define_request!(
    #[derive(Serialize)]
    EndPollRequest<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId,
            id: &'a Id,
            status: PollStatus
        };
        into_json
    }
);
