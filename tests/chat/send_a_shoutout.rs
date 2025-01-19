fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: send_a_shoutout,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageShoutouts]),
    args: [
        BroadcasterId::new("12345"),
        BroadcasterId::new("626262"),
        ModeratorId::new("98765")
    ],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/shoutouts?from_broadcaster_id=12345&to_broadcaster_id=626262&moderator_id=98765"
);
