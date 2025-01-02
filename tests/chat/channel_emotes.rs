fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: channel_emotes,
    token_type: Any,
    scopes: None,
    args: ["141981764"],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/emotes?broadcaster_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n\t\"data\": [\n\t\t{\n\t\t\t\"id\": \"304456832\",\n\t\t\t\"name\": \"twitchdevPitchfork\",\n\t\t\t\"images\": {\n\t\t\t\t\"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n\t\t\t\t\"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n\t\t\t\t\"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n\t\t\t},\n\t\t\t\"tier\": \"1000\",\n\t\t\t\"emote_type\": \"subscriptions\",\n\t\t\t\"emote_set_id\": \"301590448\",\n\t\t\t\"format\": [\"static\"],\n\t\t\t\"scale\": [\"1.0\", \"2.0\", \"3.0\"],\n\t\t\t\"theme_mode\": [\"light\", \"dark\"]\n\t\t},\n\t\t{\n\t\t\t\"id\": \"emotesv2_4c3b4ed516de493bbcd2df2f5d450f49\",\n\t\t\t\"name\": \"twitchdevHyperPitchfork\",\n\t\t\t\"images\": {\n\t\t\t\t\"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/1.0\",\n\t\t\t\t\"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/2.0\",\n\t\t\t\t\"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/3.0\"\n\t\t\t},\n\t\t\t\"tier\": \"1000\",\n\t\t\t\"emote_type\": \"subscriptions\",\n\t\t\t\"emote_set_id\": \"318939165\",\n\t\t\t\"format\": [\"static\", \"animated\"],\n\t\t\t\"scale\": [\"1.0\", \"2.0\", \"3.0\"],\n\t\t\t\"theme_mode\": [\"light\", \"dark\"]\n\t\t}\n\t],\n\t\"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
    module: twitch_highway::chat::response::EmotesResponse,
    de: EmotesResponse
);
