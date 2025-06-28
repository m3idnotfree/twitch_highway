fn_expected_request!(
    modules: [
        twitch_highway::streams::StreamsAPI,
        twitch_highway::streams::request::StreamMarkerSelector,
        twitch_highway::types::UserId,
        twitch_highway::types::PaginationQuery,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_stream_marker,
    token_type: User,
    scopes: Some(vec![Scope::UserReadBroadcast]),
    args: [
        StreamMarkerSelector::by_user_id(UserId::new("123")),
        Some(PaginationQuery::new().first(5))
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/streams/markers?user_id=123&first=5",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"123\",\n      \"user_name\": \"TwitchName\",\n      \"user_login\": \"twitchname\",\n      \"videos\": [\n        {\n          \"video_id\": \"456\",\n          \"markers\": [\n            {\n              \"id\": \"106b8d6243a4f883d25ad75e6cdffdc4\",\n              \"created_at\": \"2018-08-20T20:10:03Z\",\n              \"description\": \"hello, this is a marker!\",\n              \"position_seconds\": 244,\n              \"URL\": \"https://twitch.tv/twitchname/manager/highlighter/456?t=0h4m06s\"\n            }\n          ]\n        }\n      ]\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjoiMjk1MjA0Mzk3OjI1Mzpib29rbWFyazoxMDZiOGQ1Y\"\n  }\n}",
    module: twitch_highway::streams::response::GetStreamMarkersResponse,
    de: GetStreamMarkersResponse
);
