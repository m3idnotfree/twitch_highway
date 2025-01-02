fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: global_emotes,
    token_type: Any,
    scopes: None,
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/emotes/global",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"196892\",\n      \"name\": \"TwitchUnity\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/3.0\"\n      },\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
    module: twitch_highway::chat::response::EmotesResponse,
    de: EmotesResponse
);
