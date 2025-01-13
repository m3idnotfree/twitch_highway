use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::raid::RaidAPI,
    endpoint: cancel_raid,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRaids]),
    args: [BroadcasterId::new("12345678")],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/raids?broadcaster_id=12345678"
);
