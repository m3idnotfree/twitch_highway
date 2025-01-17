use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{GAME_ID, USER_ID},
        GameId, UserId,
    },
    IntoRequestBody,
};

request_struct!(
    #[derive(Serialize)]
    GetStreamsRequest {
        user_id	:Vec<UserId>,
        user_login:Vec<String>,
        game_id:Vec<GameId>,
        #[serde(rename="type")]
        kind: String,
        language: Vec<String>,
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
                    .map(|user| user.into_iter().map(|u| (GAME_ID, u))),
            )
            .push_opt("type", self.kind)
            .extend_opt(
                self.language
                    .map(|user| user.into_iter().map(|u| ("language", u))),
            );

        params.build()
    }
}
#[derive(Debug, Serialize)]
pub struct CreateStreamMarkerRequest {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl IntoRequestBody for CreateStreamMarkerRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct StreamMarkerFilter {
    user_id: Option<UserId>,
    video_id: Option<String>,
}

impl StreamMarkerFilter {
    pub fn by_user_id(user_id: UserId) -> Self {
        Self {
            user_id: Some(user_id),
            video_id: None,
        }
    }
    pub fn by_video_id<T: Into<String>>(video_id: T) -> Self {
        Self {
            user_id: None,
            video_id: Some(video_id.into()),
        }
    }
}

impl IntoQueryPairs for StreamMarkerFilter {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(USER_ID, self.user_id)
            .push_opt("video_id", self.video_id);

        params.build()
    }
}
request_struct!(
    #[derive(Serialize)]
        GetStreamMarkerRequest {
        string {
            video_id: String
        },
        any {
            user_id: UserId
        }
    }
);

impl IntoQueryPairs for GetStreamMarkerRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(USER_ID, self.user_id)
            .push_opt("video_id", self.video_id);

        params.build()
    }
}
