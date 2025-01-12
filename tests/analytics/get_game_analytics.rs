use twitch_highway::analytics::request::GameAnalyticsRequest;

fn_expected_request!(
    api: twitch_highway::analytics::AnalyticsAPI,
    endpoint: get_game_analytics,
    token_type: User,
    scopes: Some(vec![Scope::AnalyticsReadGames]),
    args: [GameAnalyticsRequest::new().game_id("493057").started_at("2018-01-01T00:00:00Z").ended_at("2018-03-01T00:00:00Z")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/analytics/games?game_id=493057&started_at=2018-01-01T00%3A00%3A00Z&ended_at=2018-03-01T00%3A00%3A00Z",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"game_id\" : \"493057\",\n      \"URL\" : \"https://twitch-piper-reports.s3-us-west-2.amazonaws.com/games/66170/overview/15183...\",\n      \"type\" : \"overview_v2\",\n      \"date_range\" : {\n        \"started_at\" : \"2018-01-01T00:00:00Z\",\n        \"ended_at\" : \"2018-03-01T00:00:00Z\"\n      }\n    }\n  ]\n}",
    module: twitch_highway::analytics::response::GameAnalyticsResponse,
    de: GameAnalyticsResponse
);

fn_expected_resopnse!(
    name: with_pagination,
    payload: "{\n  \"data\": [\n    {\n      \"game_id\": \"9821\",\n      \"URL\": \"https://twitch-piper-reports.s3-us-west-2.amazonaws.com/games/9821/overview/152642...\",\n      \"type\" : \"overview_v2\",\n      \"date_range\" : {\n        \"started_at\" : \"2018-03-13T00:00:00Z\",\n        \"ended_at\" : \"2018-06-13T00:00:00Z\"\n      }\n    }\n  ],\n  \"pagination\": {\"cursor\": \"eyJiIjpudWxsLJxhIjoiIn0gf5\"}\n}",
    module: twitch_highway::analytics::response::GameAnalyticsResponse,
    de: GameAnalyticsResponse
);
