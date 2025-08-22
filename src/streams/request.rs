use serde::Serialize;

use crate::types::{GameId, UserId};

define_request!(
    #[derive(Serialize)]
    GetStreamsRequest<'a> {
        opts: {
            user_id	:&'a [UserId] => USER_ID ; vec,
            user_login:&'a [&'a str] => "user_login" ; vec,
            game_id:&'a [GameId] => GAME_ID ; vec,
            #[serde(rename="type")]
            kind: &'a str => "type",
            language: &'a [&'a str] => "language" ; vec,
        };
        into_query
    }
);

define_request!(
    #[derive(Serialize)]
    CreateStreamMarkerRequest<'a> {
        req: {
            user_id: &'a UserId,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<&'a str>,
        };
        into_json
    }
);

define_select!(
    #[derive(Debug, Serialize)]
    StreamMarkerSelector<'a> {
        user_id: &'a UserId => USER_ID as by_user_id,
        video_id: &'a str as by_video_id,
    };
    into_query
);

define_request!(
    #[derive(Serialize)]
    GetStreamMarkerRequest<'a> {
        opts: {
            video_id: &'a str,
            user_id: &'a UserId => USER_ID
        }
    }
);
