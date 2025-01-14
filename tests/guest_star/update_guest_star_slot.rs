use twitch_highway::types::{BroadcasterId, ModeratorId};

fn_expected_request!(
    api: twitch_highway::guest_star::GuestStarAPI,
    endpoint: update_guest_star_slot,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageGuestStar]),
    args: [
        BroadcasterId::new("9321049"),
        ModeratorId::new("9321049"),
        "2KFRQbFtpmfyD3IevNRnCzOPRJI",
        "1",
        Some("2")
    ],
    method: PATCH,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/slot?broadcaster_id=9321049&moderator_id=9321049&session_id=2KFRQbFtpmfyD3IevNRnCzOPRJI&source_slot_id=1&destination_slot_id=2"
);
