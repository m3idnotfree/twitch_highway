use twitch_highway::{
    guest_star::{request::GustStarSettingRequest, types::GroupLayout},
    types::BroadcasterId,
};

fn_expected_request!(
    api: twitch_highway::guest_star::GuestStarAPI,
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
