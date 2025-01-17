use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{BROADCASTER_ID, GAME_ID, ID, STARTED_AT},
        BroadcasterId, GameId, Id,
    },
};

#[derive(Debug)]
pub struct ClipsFilter {
    broadcaster_id: Option<BroadcasterId>,
    game_id: Option<GameId>,
    ids: Option<Vec<Id>>,
}

impl ClipsFilter {
    pub fn by_broadcaster(broadcaster_id: BroadcasterId) -> Self {
        Self {
            broadcaster_id: Some(broadcaster_id),
            game_id: None,
            ids: None,
        }
    }
    pub fn by_game(game_id: GameId) -> Self {
        Self {
            broadcaster_id: None,
            game_id: Some(game_id),
            ids: None,
        }
    }
    pub fn by_ids(ids: Vec<Id>) -> Self {
        Self {
            broadcaster_id: None,
            game_id: None,
            ids: Some(ids),
        }
    }
}

impl IntoQueryPairs for ClipsFilter {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(BROADCASTER_ID, self.broadcaster_id)
            .push_opt(GAME_ID, self.game_id)
            .extend_opt(self.ids.map(|id| id.into_iter().map(|x| (ID, x))));

        params.build()
    }
}

request_struct!(
    GetClipsRequest {
        string  {
            started_at:String,
            ended_at:String
        },
        any {
            is_featured:bool

        }
    }
);

impl IntoQueryPairs for GetClipsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(STARTED_AT, self.started_at)
            .push_opt("ended_at", self.ended_at)
            .push_opt("is_featured", self.is_featured.map(|x| x.to_string()));

        params.build()
    }
}
