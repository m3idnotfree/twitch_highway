fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: ["123456", "654321", None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_first,
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: ["123456", "654321", Some(40), None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_after,
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: ["123456", "654321", None, Some("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_first_after,
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: ["123456", "654321", Some(40), Some("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"128393656\",\n      \"user_login\": \"smittysmithers\",\n      \"user_name\": \"smittysmithers\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  },\n  \"total\": 8\n}",
    module: twitch_highway::chat::response::ChattersResponse,
    de: ChattersResponse
);
