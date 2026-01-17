#![cfg(feature = "users")]

#[macro_use]
mod common;

use twitch_highway::{
    types::{BroadcasterId, ExtensionId, UserId},
    users::{Component, Overlay, Panel, UserAPI},
};

api_test!(build
    get_users |api| {
        api.get_users().ids(&[UserId::from("141981764")])
    }
);
api_test!(update_user, ["BaldAngel"]);
api_test!(build
    get_user_block_list |api| {
        api.get_user_block_list(&BroadcasterId::from("141981764"))
    }
);
api_test!(build
    block_user |api| {
        api.block_user(&UserId::from("198704263"))
    }
);
api_test!(unblock_user, [&UserId::from("198704263")]);
api_test!(get_user_extensions, []);
api_test!(build
    get_user_active_extensions |api| {
        api.get_user_active_extensions()
    }
);
api_test!(build
    update_user_extensions |api| {
        api.update_user_extensions()
            .add_panel(
                Panel::new(true)
                    .id(ExtensionId::from("rh6jq1q334hqc2rr1qlzqbvwlfl3x0"))
                    .version("1.1.8".to_string()),
            )
            .add_panel(
                Panel::new(true)
                    .id(ExtensionId::from("wi08ebtatdc7oj83wtl9uxwz807l8b"))
                    .version("1.1.8"),
            )
            .add_overlay(
                Overlay::new(true)
                    .id(ExtensionId::from("zfh2irvx2jb4s60f02jq0ajm8vwgka"))
                    .version("1.0.19"),
            )
            .add_component(
                Component::new(true)
                    .id(ExtensionId::from("lqnf3zxk0rv0g7gq92mtmnirjz2cjj"))
                    .version("0.0.1")
                    .name("Dev Experience Test")
                    .x(0)
                    .y(0),
            )
            .add_component(Component::new(false))
    }
);

api_test!(
    get_authorization_by_user,
    [&[UserId::from("41981764"), UserId::from("197886470")]]
);
