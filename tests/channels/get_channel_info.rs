fn_expected_request!(
    modules: [
        twitch_highway::channels::ChannelsAPI,
        twitch_highway::types::BroadcasterId
    ],
    endpoint: get_channel_info,
    token_type: Any,
    scopes: None,
    args: [&[BroadcasterId::new("141981764")]],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels?broadcaster_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_language\": \"en\",\n      \"game_id\": \"509670\",\n      \"game_name\": \"Science & Technology\",\n      \"title\": \"TwitchDev Monthly Update // May 6, 2021\",\n      \"delay\": 0,\n      \"tags\": [\"DevsInTheKnow\"],\n      \"content_classification_labels\": [\"Gambling\", \"DrugsIntoxication\", \"MatureGame\"],\n      \"is_branded_content\": false\n    }\n  ]\n}",
    module: twitch_highway::channels::response::ChannelInfoResponse,
    de: ChannelInfoResponse
);
