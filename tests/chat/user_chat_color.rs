use twitch_highway::types::UserId;

fn_expected_request!(
    api: twitch_highway::chat::ChatAPI,
    endpoint: user_chat_color,
    token_type: Any,
    scopes: None,
    args: [[UserId::new("11111"), UserId::new("44444")]],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/color?user_id=11111&user_id=44444",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"11111\",\n      \"user_name\": \"SpeedySpeedster1\",\n      \"user_login\": \"speedyspeedster1\",\n      \"color\": \"#9146FF\"\n    },\n    {\n      \"user_id\": \"44444\",\n      \"user_name\": \"SpeedySpeedster2\",\n      \"user_login\": \"speedyspeedster2\",\n      \"color\": \"\"\n    }\n  ]\n}",
    module: twitch_highway::chat::response::UsersColorResponse,
    de: UsersColorResponse
);
