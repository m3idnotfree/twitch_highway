fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: user_emotes,
    token_type: User,
    scopes: Some(vec![Scope::UserReadEmotes]),
    args: ["123456",None,None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/emotes/user?user_id=123456",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"emote_set_id\": \"\",\n      \"emote_type\": \"hypetrain\",\n      \"format\": [\n        \"static\"\n      ],\n      \"id\": \"304420818\",\n      \"name\": \"HypeLol\",\n      \"owner_id\": \"477339272\",\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\",\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLJxhIjoiIn0gf5\"\n  }\n}",
    module: twitch_highway::chat::response::EmotesResponse,
    de: EmotesResponse
);
