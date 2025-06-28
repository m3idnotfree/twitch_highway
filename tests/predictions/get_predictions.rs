fn_expected_request!(
    modules: [
        twitch_highway::predictions::PredictionsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_predictions,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadPredictions]),
    args: [
        BroadcasterId::new("55696719"),
        Some(&[Id::new("d6676d5c-c86e-44d2-bfc4-100fb48f0656")]),
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/predictions?broadcaster_id=55696719&id=d6676d5c-c86e-44d2-bfc4-100fb48f0656"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"d6676d5c-c86e-44d2-bfc4-100fb48f0656\",\n      \"broadcaster_id\": \"55696719\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"title\": \"Will there be any leaks today?\",\n      \"winning_outcome_id\": null,\n      \"outcomes\": [\n        {\n          \"id\": \"021e9234-5893-49b4-982e-cfe9a0aaddd9\",\n          \"title\": \"Yes\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"BLUE\"\n        },\n        {\n          \"id\": \"ded84c26-13cb-4b48-8cb5-5bae3ec3a66e\",\n          \"title\": \"No\",\n          \"users\": 0,\n          \"channel_points\": 0,\n          \"top_predictors\": null,\n          \"color\": \"PINK\"\n        }\n      ],\n      \"prediction_window\": 600,\n      \"status\": \"ACTIVE\",\n      \"created_at\": \"2021-04-28T16:03:06.320848689Z\",\n      \"ended_at\": null,\n      \"locked_at\": null\n    }\n  ],\n  \"pagination\": {}\n}",
    module: twitch_highway::predictions::response::PredictionsResponse,
    de: PredictionsResponse
);
