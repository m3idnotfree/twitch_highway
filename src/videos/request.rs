use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{Id, UserId, ID, USER_ID},
};

use super::types::{VideoPeriod, VideoSort, VideoType};

request_struct!(
    #[derive(Debug, Default)]
    VideosRequest {
        id:Vec<Id>,
        user_id:UserId,
        game_id:String,
        language: String,
        period: VideoPeriod,
        sort: VideoSort,
        kind: VideoType,
        first: String,
        after: String,
        before: String
    }
);

impl IntoQueryPairs for VideosRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .extend_opt(self.id.map(|ids| ids.into_iter().map(|id| (ID, id))))
            .push_opt(USER_ID, self.user_id)
            .push_opt("game_id", self.game_id)
            .push_opt("language", self.language)
            .push_opt("period", self.period.map(|x| x.to_string()))
            .push_opt("sort", self.sort.map(|x| x.to_string()))
            .push_opt("type", self.kind.map(|x| x.to_string()))
            .push_opt("first", self.first)
            .push_opt("after", self.after)
            .push_opt("before", self.before);

        params.build()
    }
}
