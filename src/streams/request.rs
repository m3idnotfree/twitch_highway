use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{UserId, AFTER, FIRST, USER_ID},
    RequestBody,
};

request_struct!(
    #[derive(Debug, Default, Serialize)]
    GetStreamsRequest {
        user_id	:Vec<UserId>,
        user_login:Vec<String>,
        game_id:Vec<String>,
        #[serde(rename="type")]
        kind: String,
        language: Vec<String>,
        first: u64,
        before: String,
        after: String
    }
);

impl IntoQueryPairs for GetStreamsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .extend_opt(
                self.user_id
                    .map(|user| user.into_iter().map(|u| (USER_ID, u))),
            )
            .extend_opt(
                self.user_login
                    .map(|user| user.into_iter().map(|u| ("user_login", u))),
            )
            .extend_opt(
                self.game_id
                    .map(|user| user.into_iter().map(|u| ("game_id", u))),
            )
            .push_opt("type", self.kind)
            .extend_opt(
                self.language
                    .map(|user| user.into_iter().map(|u| ("language", u))),
            )
            .push_opt("first", self.first.map(|x| x.to_string()))
            .push_opt("before", self.before)
            .push_opt("after", self.after);
        params.build()
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
    CreateStreamMarkerRequest {
        required {
            user_id	:UserId,
        },
        optional {
            description:String
        }
    }
);

impl RequestBody for CreateStreamMarkerRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Default, Serialize)]
    GetStreamMarkerRequest {
        user_id: UserId,
        video_id: String,
        first: u64,
        before: String,
        after: String,
    }
);

impl IntoQueryPairs for GetStreamMarkerRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(USER_ID, self.user_id)
            .push_opt("video_id", self.video_id)
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt("before", self.before)
            .push_opt(AFTER, self.after);

        params.build()
    }
}
