use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{GAME_ID, STARTED_AT, TYPE},
        GameId,
    },
};

request_struct!(
    #[derive(Serialize)]
    GameAnalyticsRequest {
        string {
            kind: String,
            started_at: String,
            ended_at: String,
        },
        any {
            game_id: GameId
        }
    }
);

impl IntoQueryPairs for GameAnalyticsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt(GAME_ID, self.game_id)
            .push_opt(TYPE, self.kind)
            .push_opt(STARTED_AT, self.started_at)
            .push_opt("ended_at", self.ended_at);

        params.build()
    }
}
