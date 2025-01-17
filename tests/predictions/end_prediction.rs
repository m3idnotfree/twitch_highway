fn_expected_request!(
    modules: [
        twitch_highway::predictions::PredictionsAPI,
        twitch_highway::predictions::types::PredictionStatus,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: end_prediction,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManagePredictions]),
    args: [
        BroadcasterId::new("141981764"),
        Id::new("bc637af0-7766-4525-9308-4112f4cbf178"),
        PredictionStatus::RESOLVED,
        Some("73085848-a94d-4040-9d21-2cb7a89374b7")
    ],
    json_contain: [
        "\"broadcaster_id\":\"141981764\"",
        "\"id\":\"bc637af0-7766-4525-9308-4112f4cbf178\"",
        "\"status\":\"RESOLVED\"",
        "\"winning_outcome_id\":\"73085848-a94d-4040-9d21-2cb7a89374b7\""
    ],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/predictions"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"bc637af0-7766-4525-9308-4112f4cbf178\",\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"title\": \"Will we win all the games?\",\n      \"winning_outcome_id\": \"73085848-a94d-4040-9d21-2cb7a89374b7\",\n      \"outcomes\": [\n        {\n          \"id\": \"73085848-a94d-4040-9d21-2cb7a89374b7\",\n          \"title\": \"yes\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"BLUE\"\n        },\n        {\n          \"id\": \"86010b2e-9764-4136-9359-fd1c9c5a8033\",\n          \"title\": \"no\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"PINK\"\n        }\n      ],\n      \"prediction_window\": 120,\n      \"status\": \"RESOLVED\",\n      \"created_at\": \"2021-04-28T21:48:19.480371331Z\",\n      \"ended_at\": \"2021-04-28T21:54:24.026833954Z\",\n      \"locked_at\": \"2021-04-28T21:48:34.636685705Z\"\n    }\n  ]\n}",
    module: twitch_highway::predictions::response::PredictionsResponse,
    de: PredictionsResponse
);
