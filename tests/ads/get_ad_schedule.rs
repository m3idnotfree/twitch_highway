fn_expected_request!(
    modules: [
        twitch_highway::ads::AdsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_ad_schedule,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadAds]),
    args: [BroadcasterId::new("123")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channels/ads?broadcaster_id=123",
    json: None,
    text: None,
    urlencoded:None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n       \"next_ad_at\" : \"2023-08-01T23:08:18+00:00\",\n      \"last_ad_at\" : \"2023-08-01T23:08:18+00:00\",\n      \"duration\" : \"60\",\n      \"preroll_free_time\" : \"90\",\n      \"snooze_count\" : \"1\",\n      \"snooze_refresh_at\" : \"2023-08-01T23:08:18+00:00\"\n    }\n  ]\n}",
    module: twitch_highway::ads::response::AdScheduleResponse,
    de: AdScheduleResponse
);
