use serde::Serialize;

use crate::types::BroadcasterId;

request_struct!(
    #[derive(Serialize)]
    StartCommercialBody {
        required {
             broadcaster_id: BroadcasterId,
             length: u64
        }
    };
    impl_body: true
);
