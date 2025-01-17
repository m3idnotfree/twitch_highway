fn_expected_request!(
    modules: [
        twitch_highway::polls::PollsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_polls,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadPolls]),
    args: [
        BroadcasterId::new("141981764"),
        Some(Id::new("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")),
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/polls?broadcaster_id=141981764&id=ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"ed961efd-8a3f-4cf5-a9d0-e616c590cd2a\",\n      \"broadcaster_id\": \"55696719\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"title\": \"Heads or Tails?\",\n      \"choices\": [\n        {\n          \"id\": \"4c123012-1351-4f33-84b7-43856e7a0f47\",\n          \"title\": \"Heads\",\n          \"votes\": 0,\n          \"channel_points_votes\": 0,\n          \"bits_votes\": 0\n        },\n        {\n          \"id\": \"279087e3-54a7-467e-bcd0-c1393fcea4f0\",\n          \"title\": \"Tails\",\n          \"votes\": 0,\n          \"channel_points_votes\": 0,\n          \"bits_votes\": 0\n        }\n      ],\n      \"bits_voting_enabled\": false,\n      \"bits_per_vote\": 0,\n      \"channel_points_voting_enabled\": false,\n      \"channel_points_per_vote\": 0,\n      \"status\": \"ACTIVE\",\n      \"duration\": 1800,\n      \"started_at\": \"2021-03-19T06:08:33.871278372Z\"\n    }\n  ],\n  \"pagination\": {}\n}",
    module: twitch_highway::polls::response::PollsResponse,
    de: PollsResponse
);
