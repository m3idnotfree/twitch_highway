fn_expected_request!(
    modules: [
        twitch_highway::guest_star::GuestStarAPI,
        twitch_highway::types::BroadcasterId ,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: send_guest_star_invites,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageGuestStar]),
    args: [
        BroadcasterId::new("9321049"),
        ModeratorId::new("9321049"),
        "2KFRQbFtpmfyD3IevNRnCzOPRJI",
        "144601104"
    ],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/invites?broadcaster_id=9321049&moderator_id=9321049&session_id=2KFRQbFtpmfyD3IevNRnCzOPRJI&guest_id=144601104"
);
