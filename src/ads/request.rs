use serde::Serialize;

use crate::types::BroadcasterId;

define_request!(
    #[derive(Serialize)]
    StartCommercialBody {
        req: {
             broadcaster_id: BroadcasterId,
             length: u64 ; u64
        };
        into_request_body
    }
);
