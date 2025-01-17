fn_expected_request!(
    modules: [
        twitch_highway::channels::ChannelsAPI,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_followed_channels,
    token_type: User,
    scopes: Some(vec![Scope::UserReadFollows]),
    args: [UserId::new("123456"), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/followed?user_id=123456",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"total\": 8,\n  \"data\": [\n    {\n      \"broadcaster_id\": \"11111\",\n      \"broadcaster_login\": \"userloginname\",\n      \"broadcaster_name\": \"UserDisplayName\",\n      \"followed_at\": \"2022-05-24T22:22:08Z\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  }\n}",
    module: twitch_highway::channels::response::FollowerdChannelsResponse,
    de: FollowerdChannelsResponse
);

fn_expected_resopnse!(
    name:empty_pagination,
    payload: "{\n  \"total\": 8,\n  \"data\": [\n    {\n      \"broadcaster_id\": \"654321\",\n      \"broadcaster_login\": \"basketweaver101\",\n      \"broadcaster_name\": \"BasketWeaver101\",\n      \"followed_at\": \"2022-05-24T22:22:08Z\"\n    }\n  ],\n  \"pagination\": {}\n}",
    module: twitch_highway::channels::response::FollowerdChannelsResponse,
    de: FollowerdChannelsResponse
);
