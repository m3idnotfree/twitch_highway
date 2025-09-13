use serde::Serialize;

use crate::types::{GameId, UserId};

define_request!(
    #[derive(Debug, Clone, Serialize)]
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
    #[derive(Debug, Clone, Serialize)]
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
    #[derive(Debug, Clone, Serialize)]
    StreamMarkerSelector<'a> {
        user_id: &'a UserId => USER_ID as by_user_id,
        video_id: &'a str as by_video_id,
    };
    into_query
);

// define_request!(
//     #[derive(Serialize)]
//     GetStreamMarkerRequest<'a> {
//         opts: {
//             video_id: &'a str,
//             user_id: &'a UserId => USER_ID
//         }
//     }
// );

#[cfg(test)]
mod tests {
    use crate::{streams::CreateStreamMarkerRequest, types::UserId};

    #[test]
    fn create_stream_marker_request() {
        let user_id = UserId::from("123");
        let req = CreateStreamMarkerRequest::new(&user_id, Some("hello, this is a marker!"))
            .into_json()
            .unwrap();

        assert_eq!(
            r#"{"user_id":"123","description":"hello, this is a marker!"}"#,
            req
        );
    }
}
