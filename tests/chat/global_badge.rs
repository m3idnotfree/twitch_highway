fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: global_badge,
    token_type: Any,
    scopes: None,
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/badges/global",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"set_id\": \"vip\",\n      \"versions\": [\n        {\n          \"id\": \"1\",\n          \"image_url_1x\": \"https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/1\",\n          \"image_url_2x\": \"https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/2\",\n          \"image_url_4x\": \"https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/3\",\n          \"title\": \"VIP\",\n          \"description\": \"VIP\",\n          \"click_action\": \"visit_url\",\n          \"click_url\": \"https://help.twitch.tv/customer/en/portal/articles/659115-twitch-chat-badges-guide\"\n        }\n      ]\n    }\n  ]\n}",
    module: twitch_highway::chat::response::BadgesResponse,
    de: BadgesResponse
);
