#![cfg(feature = "guest-star")]

#[macro_use]
mod common;

use twitch_highway::{
    guest_star::{GroupLayout, GuestStarAPI},
    types::{BroadcasterId, ModeratorId, SessionId, UserId},
};

api_test!(
    get_channel_guest_star_settings[&BroadcasterId::from("932104"), &ModeratorId::from("9321049")],
    get_guest_star_session[&BroadcasterId::from("9321049"), &ModeratorId::from("9321049")],
    create_guest_star_session [&BroadcasterId::from("9321049")],
    end_guest_star_session[&BroadcasterId::from("9321049"), &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI")],
    get_guest_star_invites[&BroadcasterId::from("9321049"), &ModeratorId::from("9321049"), &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI")],
    send_guest_star_invite[&BroadcasterId::from("9321049"), &ModeratorId::from("9321049"), &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"), &UserId::from("144601104")],
    delete_guest_star_invite[&BroadcasterId::from("9321049"), &ModeratorId::from("9321049"), &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"), &UserId::from("144601104")],
    assign_guest_star_slot[&BroadcasterId::from("9321049"), &ModeratorId::from("9321049"), &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"), &UserId::from("144601104"), "1"]
);

api_test!(
    update_channel_guest_star_settings | api | {
        api.update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
            .group_layout(GroupLayout::TILED_LAYOUT)
    }
);

api_test!(
    update_guest_star_slot | api | {
        api.update_guest_star_slot(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .destination_slot_id("2")
    }
);

api_test!(
    delete_guest_star_slot | api | {
        api.delete_guest_star_slot(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            &UserId::from("144601104"),
            "1",
        )
    }
);

api_test!(
    update_guest_star_slot_settings | api | {
        api.update_guest_star_slot_settings(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .is_audio_enabled(false)
    }
);

api_test!(
    update_channel_guest_star_settings as update_channel_guest_star_settings2 | api | {
        api.update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
            .is_moderator_send_live_enabled(false)
    }
);

api_test!(
    update_channel_guest_star_settings as update_channel_guest_star_settings3 | api | {
        api.update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
            .slot_count(6)
    }
);

api_test!(
    update_channel_guest_star_settings as update_channel_guest_star_settings4 | api | {
        api.update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
            .regenerate_browser_sources(true)
    }
);

api_test!(
    update_guest_star_slot_settings as update_guest_star_slot_settings2 | api | {
        api.update_guest_star_slot_settings(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .is_live(true)
    }
);
