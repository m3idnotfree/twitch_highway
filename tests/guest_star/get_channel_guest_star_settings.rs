use twitch_highway::types::{BroadcasterId, ModeratorId};

fn_expected_request!(
    api: twitch_highway::guest_star::GuestStarAPI,
    endpoint: get_channel_guest_star_settings,
    token_type: User,
    scopes: Some(vec![
                    Scope::ChannelReadGuestStar,
                    Scope::ChannelManageGuestStar,
                    Scope::ModeratorReadGuestStar
            ]),
    args: [BroadcasterId::new("9321049"), ModeratorId::new("9321049")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/channel_settings?broadcaster_id=9321049&moderator_id=9321049"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [        \n    { \n      \"is_moderator_send_live_enabled\": true,            \"slot_count\": 4,            \"is_browser_source_audio_enabled\": true,            \"group_layout\": \"TILED_LAYOUT\",            \"browser_source_token\": \"eihq8rew7q3hgierufhi3q\"        \n    }    \n  ]\n}",
    module: twitch_highway::guest_star::response::GuestStarSettingsResponse,
    de: GuestStarSettingsResponse
);
