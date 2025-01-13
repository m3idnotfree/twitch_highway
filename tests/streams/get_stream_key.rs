use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::streams::StreamsAPI,
    endpoint: get_stream_key,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadStreamKey]),
    args: [BroadcasterId::new("149747285")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/streams/key?broadcaster_id=149747285",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"stream_key\": \"live_44322889_a34ub37c8ajv98a0\"\n    }\n  ]\n}",
    module: twitch_highway::streams::response::StreamKeyResponse,
    de: StreamKeyResponse
);
