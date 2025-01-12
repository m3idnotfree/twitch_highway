use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{BroadcasterId, Id, AFTER, BROADCASTER_ID, FIRST, ID, STARTED_AT},
};

request_struct!(
    #[derive(Debug)]
    GetClipsRequest {
        required {
            broadcaster_id:BroadcasterId,
            game_id:String,
            id:Vec<Id>
        },
        optional {
            started_at:String,
            ended_at:String,
            first:u64,
            before:String,
            after:String,
            is_featured:bool
        }
    }
);

impl IntoQueryPairs for GetClipsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push(BROADCASTER_ID, self.broadcaster_id)
            .push("game_id", self.game_id)
            .extend(self.id.into_iter().map(|x| (ID, x)))
            .push_opt(STARTED_AT, self.started_at)
            .push_opt("ended_at", self.ended_at)
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt("before", self.before)
            .push_opt(AFTER, self.after)
            .push_opt("is_featured", self.is_featured.map(|x| x.to_string()));

        params.build()
    }
}
