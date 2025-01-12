use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::channels::ChannelsAPI,
    endpoint: get_channel_editors,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadEditors]),
    args: [BroadcasterId::new("141981764")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/editors?broadcaster_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"182891647\",\n      \"user_name\": \"mauerbac\",\n      \"created_at\": \"2019-02-15T21:19:50.380833Z\"\n    },\n    {\n      \"user_id\": \"135093069\",\n      \"user_name\": \"BlueLava\",\n      \"created_at\": \"2018-03-07T16:28:29.872937Z\"\n    }\n  ]\n}",
    module: twitch_highway::channels::response::ChannelEditorsResponse,
    de: ChannelEditorsResponse
);
