use serde::Serialize;

use crate::{types::BroadcasterId, AsBody};

request_struct!(
    #[derive(Debug, Serialize)]
    StartCommercialRequest {
        required {
            broadcaster_id: BroadcasterId,
            length: u64
        }
    }
);

impl AsBody for StartCommercialRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
