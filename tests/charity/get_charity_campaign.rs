fn_expected_request!(
    modules: [
        twitch_highway::charity::CharityAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_charity_campaign,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadCharity]),
    args: [BroadcasterId::new("123456")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/charity/campaigns?broadcaster_id=123456",
    json: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"123-abc-456-def\",\n      \"broadcaster_id\": \"123456\",\n      \"broadcaster_name\": \"SunnySideUp\",\n      \"broadcaster_login\": \"sunnysideup\",\n      \"charity_name\": \"Example name\",\n      \"charity_description\": \"Example description\",\n      \"charity_logo\": \"https://abc.cloudfront.net/ppgf/1000/100.png\",\n      \"charity_website\": \"https://www.example.com\",\n      \"current_amount\": {\n        \"value\": 86000,\n        \"decimal_places\": 2,\n        \"currency\": \"USD\"\n      },\n      \"target_amount\": {\n        \"value\": 1500000,\n        \"decimal_places\": 2,\n        \"currency\": \"USD\"\n      }\n    }\n  ]\n}",
    module: twitch_highway::charity::response::CharityCampaignResponse,
    de: CharityCampaignResponse
);
