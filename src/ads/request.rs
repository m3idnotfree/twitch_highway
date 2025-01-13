use serde::Serialize;

use crate::{types::BroadcasterId, RequestBody};

request_struct!(
    #[derive(Debug, Serialize)]
    StartCommercialRequestBody {
        required {
             broadcaster_id: BroadcasterId,
             length: u64
        }
    }
);

impl RequestBody for StartCommercialRequestBody {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
