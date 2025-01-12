use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::clips::ClipsAPI,
    endpoint: create_clip,
    token_type: Any,
    scopes: None,
    args: [BroadcasterId::new("44322889"), None],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/clips?broadcaster_id=44322889",
    json: None
);
