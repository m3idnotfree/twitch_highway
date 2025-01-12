use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api:twitch_highway::ads::AdsAPI,
    endpoint: snooze_next_ad,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageAds]),
    args: [BroadcasterId::new("123")],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/ads/schedule/snooze?broadcaster_id=123",
    json: None,
    text: None,
    urlencoded:None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"snooze_count\" : \"1\",\n      \"snooze_refresh_at\" : \"2023-08-01T23:08:18+00:00\",\n      \"next_ad_at\" : \"2023-08-01T23:08:18+00:00\"\n    }\n  ]}",
    module: twitch_highway::ads::response::SnoozeNextAdResponse,
    de: SnoozeNextAdResponse
);
