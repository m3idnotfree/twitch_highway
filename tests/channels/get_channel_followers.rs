use twitch_highway::{channels::request::ChannelFollowRequest, types::UserId};

fn_expected_request!(
    api: twitch_highway::channels::ChannelsAPI,
    endpoint: get_channel_followers,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadFollowers]),
    args: [ChannelFollowRequest::new(UserId::new("123456"))],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/followed?user_id=123456",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"total\": 8,\n  \"data\": [\n    {\n      \"broadcaster_id\": \"11111\",\n      \"broadcaster_login\": \"userloginname\",\n      \"broadcaster_name\": \"UserDisplayName\",\n      \"followed_at\": \"2022-05-24T22:22:08Z\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  }\n}",
    module: twitch_highway::channels::response::ChannelFollowRequestResponse,
    de: ChannelFollowRequestResponse
);

fn_expected_resopnse!(
    name:empty_pagination,
    payload: "{\n  \"total\": 8,\n  \"data\": [],\n  \"pagination\": {}\n}",
    module: twitch_highway::channels::response::ChannelFollowRequestResponse,
    de: ChannelFollowRequestResponse
);
