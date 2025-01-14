use twitch_highway::types::{BroadcasterId, ModeratorId};

fn_expected_request!(
    api: twitch_highway::guest_star::GuestStarAPI,
    endpoint: delete_guest_star_slot,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageGuestStar]),
    args: [
        BroadcasterId::new("9321049"),
        ModeratorId::new("9321049"),
        "2KFRQbFtpmfyD3IevNRnCzOPRJI",
        "144601104",
        "1",
        None
    ],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/slot?broadcaster_id=9321049&moderator_id=9321049&session_id=2KFRQbFtpmfyD3IevNRnCzOPRJI&guest_id=144601104&slot_id=1"
);
