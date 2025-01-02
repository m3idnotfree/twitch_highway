fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: emote_sets,
    token_type: Any,
    scopes: None,
    args: [["1234"]],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: id_vec,
    api:twitch_highway::chat::ChatAPI,
    endpoint: emote_sets,
    token_type: Any,
    scopes: None,
    args: [["1234", "4567"]],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234&emote_set_id=4567",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"304456832\",\n      \"name\": \"twitchdevPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n      },\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"301590448\",\n      \"owner_id\": \"141981764\",\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
    module: twitch_highway::chat::response::EmotesResponse,
    de: EmotesResponse
);
