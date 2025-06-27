use serde::Serialize;

use crate::types::{GameId, UserId};

define_request!(
    #[derive(Serialize)]
    GetStreamsRequest<'a> {
        opts: {
            user_id	:Vec<UserId> => USER_ID ; vec,
            user_login:Vec<&'a str> => "user_login" ; vec,
            game_id:Vec<GameId> => GAME_ID ; vec,
            #[serde(rename="type")]
            kind: &'a str => TYPE,
            language: Vec<&'a str> => "language" ; vec,
        };
        apply_to_url
    }
);

define_request!(
    #[derive(Serialize)]
    CreateStreamMarkerRequest {
        req: {
            user_id: UserId,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        };
        into_request_body
    }
);

define_select!(
    #[derive(Debug, Serialize)]
    StreamMarkerFilter<'a> {
        user_id: UserId => USER_ID as by_user_id,
        video_id: &'a str as by_video_id,
    };
    apply_to_url
);

define_request!(
    #[derive(Serialize)]
    GetStreamMarkerRequest<'a> {
        opts: {
            video_id: &'a str,
            user_id: UserId => USER_ID
        }
    }
);
