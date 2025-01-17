fn_expected_request!(
    modules: [
        twitch_highway::polls::PollsAPI,
        twitch_highway::polls::request::PollsRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Title,
        twitch_oauth_token::types::Scope
    ],
    endpoint: create_poll,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManagePolls]),
    args: [
        BroadcasterId::new("141981764"),
        "Heads or Tails?",
        vec![Title::new("Heads".to_string()), Title::new("Tails".to_string())],
        1800,
        Some(
            PollsRequest::new()
                .channel_points_voting_enabled(true)
                .channel_points_per_vote(100)
        )
    ],
    json_contain: [
        "\"broadcaster_id\":\"141981764\"",
        "\"title\":\"Heads or Tails?\"",
        "\"channel_points_voting_enabled\":true",
        "\"choices\":[{\"title\":\"Heads\"},{\"title\":\"Tails\"}]",
        "\"channel_points_per_vote\":100",
        "\"duration\":1800"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/polls"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"ed961efd-8a3f-4cf5-a9d0-e616c590cd2a\",\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"title\": \"Heads or Tails?\",\n      \"choices\": [\n        {\n          \"id\": \"4c123012-1351-4f33-84b7-43856e7a0f47\",\n          \"title\": \"Heads\",\n          \"votes\": 0,\n          \"channel_points_votes\": 0,\n          \"bits_votes\": 0\n        },\n        {\n          \"id\": \"279087e3-54a7-467e-bcd0-c1393fcea4f0\",\n          \"title\": \"Tails\",\n          \"votes\": 0,\n          \"channel_points_votes\": 0,\n          \"bits_votes\": 0\n        }\n      ],\n      \"bits_voting_enabled\": false,\n      \"bits_per_vote\": 0,\n      \"channel_points_voting_enabled\": true,\n      \"channel_points_per_vote\": 100,\n      \"status\": \"ACTIVE\",\n      \"duration\": 1800,\n      \"started_at\": \"2021-03-19T06:08:33.871278372Z\"\n    }\n  ]\n}",
    module: twitch_highway::polls::response::PollsResponse,
    de: PollsResponse
);
