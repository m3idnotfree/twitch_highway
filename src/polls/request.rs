use serde::Serialize;

use crate::{
    types::{BroadcasterId, Id, Title},
    RequestBody,
};

use super::types::PollStatus;

request_struct!(
    #[derive(Debug,Serialize)]
    PollsRequest {
    required {
        broadcaster_id: BroadcasterId,
        title: String,
        choices:Vec<Title>,
        duration:u64
    },
    optional {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_voting_enabled:bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_per_vote:u64,

    }
}
);

impl RequestBody for PollsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    #[derive(Debug,Serialize)]
    EndPollRequest {
        required {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PollStatus
        }
    }
);

impl RequestBody for EndPollRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
