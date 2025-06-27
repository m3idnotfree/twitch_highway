use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::types::UserId;

define_request!(
    #[derive(Serialize)]
    BitsLeaderboardRequest {
        opts: {
            count: u64 ; u64,
            period: String,
            started_at: DateTime<FixedOffset> => STARTED_AT ; date,
            user_id: UserId => USER_ID,
        };
        apply_to_url
    }
);
