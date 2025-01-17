fn_expected_request!(
    modules: [
        twitch_highway::streams::StreamsAPI,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: create_stream_marker,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageBroadcast]),
    args: [UserId::new("123"), Some("hello, this is a marker!".to_string())],
    json_contain: ["\"user_id\":\"123\"","\"description\":\"hello, this is a marker!\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/streams/markers"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n     {\n        \"id\": \"123\",\n        \"created_at\": \"2018-08-20T20:10:03Z\",\n        \"description\": \"hello, this is a marker!\",\n        \"position_seconds\": 244\n     }\n  ]\n}",
    module: twitch_highway::streams::response::CreateStreamMarkerResponse,
    de: CreateStreamMarkerResponse
);
