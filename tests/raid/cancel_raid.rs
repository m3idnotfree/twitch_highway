fn_expected_request!(
    modules: [
        twitch_highway::raid::RaidAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: cancel_raid,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRaids]),
    args: [BroadcasterId::new("12345678")],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/raids?broadcaster_id=12345678"
);
