fn_expected_request!(
    modules: [
        twitch_highway::guest_star::GuestStarAPI,
        twitch_highway::guest_star::request::UpdateSlotSettingsRequest,
        twitch_highway::types::BroadcasterId ,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_guest_star_slot_settings,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageGuestStar]),
    args: [
        BroadcasterId::new("9321049"),
        ModeratorId::new("9321049"),
        "2KFRQbFtpmfyD3IevNRnCzOPRJI",
        "1",
        UpdateSlotSettingsRequest::new().is_audio_enabled(false)
    ],
    method: PATCH,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/slot_settings?broadcaster_id=9321049&moderator_id=9321049&session_id=2KFRQbFtpmfyD3IevNRnCzOPRJI&slot_id=1&is_audio_enabled=false"
);
