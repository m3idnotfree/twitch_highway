use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::hype_train::HypeTrainAPI,
    endpoint: get_hype_train_events,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadHypeTrain]),
    args: [
        BroadcasterId::new("270954519"),
        Some(1),
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/hypetrain/events?broadcaster_id=270954519&first=1"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"1b0AsbInCHZW2SQFQkCzqN07Ib2\",\n      \"event_type\": \"hypetrain.progression\",\n      \"event_timestamp\": \"2020-04-24T20:07:24Z\",\n      \"version\": \"1.0\",\n      \"event_data\": {\n        \"broadcaster_id\": \"270954519\",\n        \"cooldown_end_time\": \"2020-04-24T20:13:21.003802269Z\",\n        \"expires_at\": \"2020-04-24T20:12:21.003802269Z\",\n        \"goal\": 1800,\n        \"id\": \"70f0c7d8-ff60-4c50-b138-f3a352833b50\",\n        \"last_contribution\": {\n          \"total\": 200,\n          \"type\": \"BITS\",\n          \"user\": \"134247454\"\n        },\n        \"level\": 2,\n        \"started_at\": \"2020-04-24T20:05:47.30473127Z\",\n        \"top_contributions\": [\n          {\n            \"total\": 600,\n            \"type\": \"BITS\",\n            \"user\": \"134247450\"\n          }\n        ],\n        \"total\": 600\n      }\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjI3MDk1NDUxOToxNTg3NzU4ODQ0OjFiMEFzYkluQ0haVzJTUUZRa0N6cU4wN0liMiJ9fQ\"\n  }\n}",
    module: twitch_highway::hype_train::response::HypeTrainResponse,
    de: HypeTrainResponse
);
