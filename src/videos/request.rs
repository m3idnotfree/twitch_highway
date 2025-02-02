use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{GAME_ID, ID, TYPE, USER_ID},
        GameId, Id, UserId,
    },
};

use super::types::{Period, Sort, Type};

pub struct VideoFilter {
    ids: Option<Vec<Id>>,
    user_id: Option<UserId>,
    game_id: Option<GameId>,
}

impl VideoFilter {
    pub fn by_ids(ids: Vec<Id>) -> Self {
        Self {
            ids: Some(ids),
            user_id: None,
            game_id: None,
        }
    }
    pub fn by_user_id(user_id: UserId) -> Self {
        Self {
            ids: None,
            user_id: Some(user_id),
            game_id: None,
        }
    }
    pub fn by_game_id(game_id: GameId) -> Self {
        Self {
            ids: None,
            user_id: None,
            game_id: Some(game_id),
        }
    }
}

impl IntoQueryPairs for VideoFilter {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .extend_opt(self.ids.map(|id| id.into_iter().map(|x| (ID, x))))
            .push_opt(USER_ID, self.user_id)
            .push_opt(GAME_ID, self.game_id);

        params.build()
    }
}

request_struct!(
    VideosRequest {
        string {
            language: String,
        },
        any {
            period: Period,
            sort: Sort,
            kind: Type,
        }
    }
);

impl IntoQueryPairs for VideosRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt("language", self.language)
            .push_opt("period", self.period.map(|x| x.to_string()))
            .push_opt("sort", self.sort.map(|x| x.to_string()))
            .push_opt(TYPE, self.kind.map(|x| x.to_string()));

        params.build()
    }
}
