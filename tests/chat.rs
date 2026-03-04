#[macro_use]
mod common;

use twitch_highway::{
    chat::{AnnouncementColor, ChatAPI, ChatColor},
    types::{BroadcasterId, ModeratorId, UserId},
};

api_test!(
    get_chatters | api | {
        api.get_chatters(&BroadcasterId::from("123456"), &ModeratorId::from("654321"))
    }
);

api_test!(
    get_channel_emotes[&BroadcasterId::from("141981764")],
    get_global_emotes [],
    get_emote_sets[&["301590448"]],
    get_channel_chat_badges[&BroadcasterId::from("135093069")],
    get_global_chat_badges [],
    get_shared_chat_session[&BroadcasterId::from("198704263")],
    send_a_shoutout[&BroadcasterId::from("12345"), &BroadcasterId::from("626262"), &ModeratorId::from("98765")],
    get_user_chat_color[&[UserId::from("11111"), UserId::from("44444")]],
    update_user_chat_color[&UserId::from("123"), ChatColor::Blue],
    // update_user_chat_color as update_user_chat_color2[&UserId::from("123"), ChatColor::Blue]
);

api_test!(get_chat_settings | api | { api.get_chat_settings(&BroadcasterId::from("1234")) });

api_test!(get_user_emotes | api | { api.get_user_emotes(&UserId::from("123456")) });

api_test!(
    update_chat_settings | api | {
        api.update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .follower_mode(false)
    }
);

api_test!(
    send_chat_announcement | api | {
        api.send_chat_announcement(
            &BroadcasterId::from("11111"),
            &ModeratorId::from("44444"),
            "Hello chat!",
        )
        .color(AnnouncementColor::Purple)
    }
);

api_test!(
    send_chat_message | api | {
        api.send_chat_message(
            &BroadcasterId::from("12826"),
            &UserId::from("141981764"),
            "Hello, world! twitchdevHype",
        )
        .for_source_only(true)
    }
);

api_test!(
    update_chat_settings as update_chat_settings2 | api | {
        api.update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .slow_mode(true)
            .slow_mode_wait_time(10)
    }
);
