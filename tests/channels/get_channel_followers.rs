fn_expected_request!(
    modules: [
        twitch_highway::channels::ChannelsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_channel_followers,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadFollowers]),
    args: [None, BroadcasterId::new("123456"), None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/followers?broadcaster_id=123456",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"total\": 8,\n  \"data\": [\n    {\n      \"user_id\": \"11111\",\n      \"user_name\": \"UserDisplayName\",\n      \"user_login\": \"userloginname\",\n      \"followed_at\": \"2022-05-24T22:22:08Z\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  }\n}",
    module: twitch_highway::channels::response::ChannelFollowersResponse,
    de: ChannelFollowersResponse
);

fn_expected_resopnse!(
    name:empty_pagination,
    payload: "{\n  \"total\": 8,\n  \"data\": [],\n  \"pagination\": {}\n}",
    module: twitch_highway::channels::response::ChannelFollowersResponse,
    de: ChannelFollowersResponse
);
