#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::types::BlockedTermId;
use twitch_highway::{
    moderation::{
        AutoModAction, CheckAutoMod, ModerationAPI, SuspiciousStatus, UnbanRequestStatus,
    },
    types::{BroadcasterId, ModeratorId, UserId},
};

api_test!(
    check_automod_status[&BroadcasterId::from("12345"), &[CheckAutoMod::new("123", "Hello World!"), CheckAutoMod::new("393", "Boooooo!")]],
    manage_held_automod_messages[&UserId::from("9327994"), "836013710", AutoModAction::ALLOW],
    get_automod_settings[&BroadcasterId::from("1234"), &ModeratorId::from("5678")],
    unban_user[&BroadcasterId::from("1234"), &ModeratorId::from("5678"), &UserId::from("5432")],
    add_blocked_term[&BroadcasterId::from("1234"), &ModeratorId::from("5678"), "A phrase I’m not fond of"],
    remove_blocked_term[&BroadcasterId::from("1234"), &ModeratorId::from("5678"), &BlockedTermId::from("c9fc79b8-0f63-4ef7-9d38-efd811e74ac2")],
    add_channel_moderator[&BroadcasterId::from("11111"), &UserId::from("44444")],
    remove_channel_moderator[&BroadcasterId::from("11111"), &UserId::from("44444")],
    add_channel_vip[&BroadcasterId::from("123"), &UserId::from("456")],
    remove_channel_vip[&BroadcasterId::from("123"), &UserId::from("456")],
    update_shield_mode_status[&BroadcasterId::from("12345"), &ModeratorId::from("98765"), false],
    get_shield_mode_status[&BroadcasterId::from("12345"), &ModeratorId::from("98765")],
    warn_chat_user[&BroadcasterId::from("404040"), &ModeratorId::from("404041"), &UserId::from("9876"), "stop doing that!"],
    add_blocked_term as add_blocked_term2[&BroadcasterId::from("1234"), &ModeratorId::from("5678"), "crac*"]
);

api_test!(
    update_automod_settings | api | {
        api.update_automod_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .overall_level(3)
    }
);

api_test!(get_banned_users | api | { api.get_banned_users(&BroadcasterId::from("198704263")) });

api_test!(
    ban_user | api | {
        api.ban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("9876"),
        )
        .reason("no reason")
    }
);

api_test!(
    get_unban_requests | api | {
        api.get_unban_requests(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("274637212"),
            UnbanRequestStatus::Pending,
        )
    }
);

api_test!(
    resolve_unban_request | api | {
        api.resolve_unban_request(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("987654321"),
            "92af127c-7326-4483-a52b-b0daa0be61c01",
            UnbanRequestStatus::Approved,
        )
    }
);

api_test!(
    get_blocked_terms | api | {
        api.get_blocked_terms(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .first(10)
    }
);

api_test!(
    delete_chat_messages | api | {
        api.delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
            .message_id("abc-123-def")
    }
);

api_test!(get_moderated_channels | api | { api.get_moderated_channels(&UserId::from("931931")) });

api_test!(get_moderators | api | { api.get_moderators(&BroadcasterId::from("198704263")) });

api_test!(get_vips | api | { api.get_vips(&BroadcasterId::from("123")) });

api_test!(
    ban_user as ban_user2 | api | {
        api.ban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("9876"),
        )
        .duration(300)
        .reason("no reason")
    }
);

#[tokio::test]
async fn ban_user_error() {
    let suite = HttpMock::new().await;

    suite.ban_user_error().await;

    let respnose = suite
        .execute(|api| {
            api.ban_user(
                &BroadcasterId::from("1234"),
                &ModeratorId::from("5678"),
                &UserId::from("9876"),
            )
            .duration(300)
            .reason("no reason")
            .build()
        })
        .json()
        .await;

    assert!(respnose.is_err());
}

#[tokio::test]
async fn unban_user_error() {
    let suite = HttpMock::new().await;

    suite.unban_user_error().await;

    let respnose = suite
        .execute(|api| {
            api.unban_user(
                &BroadcasterId::from("1234"),
                &ModeratorId::from("5678"),
                &UserId::from("5432"),
            )
        })
        .json()
        .await;

    assert!(respnose.is_err());
}

api_test!(
    delete_chat_messages as delete_chat_messages2 | api | {
        api.delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
            .message_id("abc-123-def")
    }
);

api_test!(
    get_vips as get_vips2 | api | {
        api.get_vips(&BroadcasterId::from("123"))
            .user_ids(&[UserId::from("456"), UserId::from("678")])
    }
);

api_test!(
    add_suspicious_status_to_chat_user [
        &BroadcasterId::from("141981764"),
        &ModeratorId::from("12826"),
        &UserId::from("9876"),
        SuspiciousStatus::Restricted
    ]
);

api_test!(
    remove_suspicious_status_from_chat_user [
        &BroadcasterId::from("141981764"),
        &ModeratorId::from("12826"),
        &UserId::from("9876"),
    ]
);
