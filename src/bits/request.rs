use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{STARTED_AT, USER_ID},
        UserId,
    },
};

request_struct!(
    #[derive(Serialize)]
    BitsLeaderboardRequest {
        string {
            period: String,
        },
        any {
            started_at: DateTime<FixedOffset>,
            user_id: UserId,
            count: u64
        }
    }
);

impl IntoQueryPairs for BitsLeaderboardRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt("count", self.count.map(|x| x.to_string()))
            .push_opt("period", self.period)
            .date_opt(STARTED_AT, self.started_at)
            .push_opt(USER_ID, self.user_id);
        params.build()
    }
}
