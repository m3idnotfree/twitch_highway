fn_expected_request!(
    modules: [
        twitch_highway::clips::ClipsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: create_clip,
    token_type: User,
    scopes: Some(vec![Scope::ClipsEdit]),
    args: [BroadcasterId::new("44322889"), None],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/clips?broadcaster_id=44322889",
    json: None
);
