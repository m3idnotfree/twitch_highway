use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::charity::CharityAPI,
    endpoint: get_charity_campaign_donations,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadCharity]),
    args: [BroadcasterId::new("123456"), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/charity/donations?broadcaster_id=123456",
    json: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"a1b2c3-aabb-4455-d1e2f3\",\n      \"campaign_id\": \"123-abc-456-def\",\n      \"user_id\": \"5678\",\n      \"user_login\": \"cool_user\",\n      \"user_name\": \"Cool_User\",\n      \"amount\": {\n        \"value\": 500,\n        \"decimal_places\": 2,\n        \"currency\": \"USD\"\n      }\n    },\n    {\n      \"id\": \"z1y2x3-ccdd-6677-d1e2f3\",\n      \"campaign_id\": \"123-abc-456-def\",\n      \"user_id\": \"8765\",\n      \"user_login\": \"cool_user2\",\n      \"user_name\": \"Cool_User2\",\n      \"amount\": {\n        \"value\": 10000,\n        \"decimal_places\": 2,\n        \"currency\": \"USD\"\n      }\n    }\n  ],\n  \"pagination\" : {\n      \"cursor\" : \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  }\n}",
    module: twitch_highway::charity::response::CharityCampaignDonationResponse,
    de: CharityCampaignDonationResponse
);
