fn_expected_request!(
    api: twitch_highway::raid::RaidAPI,
    endpoint: start_raid,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRaids]),
    args: ["12345678","87654321"],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/raids?from_broadcaster_id=12345678&to_broadcaster_id=87654321"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"created_at\": \"2022-02-18T07:20:50.52Z\",\n      \"is_mature\": false\n    }\n  ]\n}",
    module: twitch_highway::raid::response::StartRaidResponse,
    de: StartRaidResponse
);
