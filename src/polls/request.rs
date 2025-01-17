use serde::Serialize;

use crate::{
    types::{BroadcasterId, Id},
    IntoRequestBody,
};

use super::types::PollStatus;

request_struct!(
    #[derive(Serialize)]
    PollsRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_voting_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        channel_points_per_vote: u64,
    }
);

impl IntoRequestBody for PollsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
    EndPollRequest {
        required {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PollStatus
        }
    }
);

impl IntoRequestBody for EndPollRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
