fn_expected_request!(
    modules: [
        twitch_highway::predictions::PredictionsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types:: Title,
        twitch_oauth_token::types::Scope
    ],
    endpoint: create_prediction,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManagePredictions]),
    args: [
        BroadcasterId::new("141981764"),
        "Any leeks in the stream?",
        vec![Title::new("Yes, give it time.".to_string()), Title::new("Definitely not.".to_string())],
        120
    ],
    json_contain: [
        "\"broadcaster_id\":\"141981764\"",
        "\"title\":\"Any leeks in the stream?\"",
        "\"outcomes\":[{\"title\":\"Yes, give it time.\"},{\"title\":\"Definitely not.\"}]",
        "\"prediction_window\":120"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/predictions"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"bc637af0-7766-4525-9308-4112f4cbf178\",\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"title\": \"Any leeks in the stream?\",\n      \"winning_outcome_id\": null,\n      \"outcomes\": [\n        {\n          \"id\": \"73085848-a94d-4040-9d21-2cb7a89374b7\",\n          \"title\": \"Yes, give it time.\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"BLUE\"\n        },\n        {\n          \"id\": \"906b70ba-1f12-47ea-9e95-e5f93d20e9cc\",\n          \"title\": \"Definitely not.\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"PINK\"\n        }\n      ],\n      \"prediction_window\": 120,\n      \"status\": \"ACTIVE\",\n      \"created_at\": \"2021-04-28T17:11:22.595914172Z\",\n      \"ended_at\": null,\n      \"locked_at\": null\n    }\n  ]\n}",
    module: twitch_highway::predictions::response::PredictionsResponse,
    de: PredictionsResponse
);
