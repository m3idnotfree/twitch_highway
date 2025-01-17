fn_expected_request!(
    modules: [twitch_highway::search::SearchAPI],
    endpoint: search_channels,
    token_type: Any,
    scopes: None,
    args: ["twitchdev", None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/search/channels?query=twitchdev"
);

fn_expected_request!(
    name: request_url_encoded,
    modules: [twitch_highway::search::SearchAPI],
    endpoint: search_channels,
    token_type: Any,
    scopes: None,
    args: ["#archery", None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/search/channels?query=%23archery"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_language\": \"en\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"display_name\": \"TwitchDev\",\n      \"game_id\": \"1469308723\",\n      \"game_name\": \"Software and Game Development\",\n      \"id\": \"141981764\",\n      \"is_live\": false,\n      \"tag_ids\": [],\n      \"tags\": [\n        \"WebDevelopment\",\n        \"GameDevelopment\",\n        \"SoftwareDevelopment\",\n        \"English\"\n      ],\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png\",\n      \"title\": \"Standard Output\",\n      \"started_at\": \"\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"Mg==\"\n  }\n}",
    module: twitch_highway::search::response::ChannelsResponse,
    de: ChannelsResponse
);
