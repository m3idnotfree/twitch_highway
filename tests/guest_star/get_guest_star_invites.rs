fn_expected_request!(
    modules: [
        twitch_highway::guest_star::GuestStarAPI,
        twitch_highway::types::BroadcasterId ,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_guest_star_invites,
    token_type: User,
    scopes: Some(vec![
        Scope::ChannelReadGuestStar,
        Scope::ChannelManageGuestStar,
        Scope::ModeratorReadGuestStar
    ]),
    args: [
        BroadcasterId::new("9321049"),
        ModeratorId::new("9321049"),
        "2KFRQbFtpmfyD3IevNRnCzOPRJI"
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/invites?broadcaster_id=9321049&moderator_id=9321049&session_id=2KFRQbFtpmfyD3IevNRnCzOPRJI"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"144601104\",\n      \"invited_at\": \"2023-01-02T04:16:53.325Z\",\n      \"status\": \"INVITED\",\n      \"is_audio_enabled\": false,\n      \"is_video_enabled\": true,\n      \"is_audio_available\": true,\n      \"is_video_available\": true\n    }\n  ]\n}",
    module: twitch_highway::guest_star::response::GustStarInvitesResponse,
    de: GustStarInvitesResponse
);
