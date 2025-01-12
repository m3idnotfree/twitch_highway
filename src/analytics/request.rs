use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{AFTER, FIRST, KIND, STARTED_AT},
};

new_request_struct!(
    #[derive(Debug, Default, Serialize)]
    GameAnalyticsRequest {
        string: {
            game_id: String,
            kind: String,
            started_at: String,
            ended_at: String,
            after: String
        },
        any: {
           first: u64
        }
    }
);

impl IntoQueryPairs for GameAnalyticsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt("game_id", self.game_id)
            .push_opt(KIND, self.kind)
            .push_opt(STARTED_AT, self.started_at)
            .push_opt("ended_at", self.ended_at)
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt(AFTER, self.after);

        params.build()
    }
}
