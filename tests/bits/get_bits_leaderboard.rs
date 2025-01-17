fn_expected_request!(
    modules: [
        twitch_highway::bits::BitsAPI,
        twitch_highway::bits::request::BitsLeaderboardRequest,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_bits_leaderboard,
    token_type: User,
    scopes: Some(vec![Scope::BitsRead]),
    args: [Some(BitsLeaderboardRequest::new().count(2).period("week".to_string()).started_at("2018-02-05T08:00:00Z".to_string()))],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/bits/leaderboard?count=2&period=week&started_at=2018-02-05T08%3A00%3A00Z",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"158010205\",\n      \"user_login\": \"tundracowboy\",\n      \"user_name\": \"TundraCowboy\",\n      \"rank\": 1,\n      \"score\": 12543\n    },\n    {\n      \"user_id\": \"7168163\",\n      \"user_login\": \"topramens\",\n      \"user_name\": \"Topramens\",\n      \"rank\": 2,\n      \"score\": 6900\n    }\n  ],\n  \"date_range\": {\n    \"started_at\": \"2018-02-05T08:00:00Z\",\n    \"ended_at\": \"2018-02-12T08:00:00Z\"\n  },\n  \"total\": 2\n}",
    module: twitch_highway::bits::response::BitsLeaderboardResponse,
    de: BitsLeaderboardResponse
);
