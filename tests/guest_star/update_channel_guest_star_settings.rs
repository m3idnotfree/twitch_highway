fn_expected_request!(
    modules: [
        twitch_highway::guest_star::GuestStarAPI,
        twitch_highway::guest_star::request::GustStarSettingRequest,
        twitch_highway::guest_star::types::GroupLayout,
        twitch_highway::types::BroadcasterId ,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_channel_guest_star_settings,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageGuestStar]),
    args: [
        BroadcasterId::new("9321049"),
        GustStarSettingRequest::new()
            .group_layout(GroupLayout::TILED_LAYOUT)
    ],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/channel_settings?broadcaster_id=9321049&group_layout=TILED_LAYOUT"
);
