#![cfg(feature = "moderation")]

#[macro_use]
mod common;

use anyhow::Result;
use common::HttpMock;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    moderation::{AutoModAction, CheckAutoMod, ModerationAPI, UnbanRequestStatus},
    types::{BroadcasterId, Id, ModeratorId, UserId},
};
use twitch_oauth_token::scope::ModerationScopes;

api_test!(
    check_automod_status,
    [
        &BroadcasterId::from("12345"),
        &[
            CheckAutoMod::new("123", "Hello World!"),
            CheckAutoMod::new("393", "Boooooo!"),
        ]
    ]
);
api_test!(
    manage_held_automod_messages,
    [&UserId::from("9327994"), "836013710", AutoModAction::ALLOW,]
);
api_test!(
    get_automod_settings,
    [&BroadcasterId::from("1234"), &ModeratorId::from("5678"),]
);
api_test!(build
    update_automod_settings |api| {
        api.update_automod_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .overall_level(3)
    }
);
api_test!(build
    get_banned_users |api| {
        api.get_banned_users(&BroadcasterId::from("198704263"))
    }
);
api_test!(build
    ban_user |api| {
        api.ban_user(&BroadcasterId::from("1234"), &ModeratorId::from("5678"), &UserId::from("9876"))
            .reason("no reason")
    }
);
api_test!(
    unban_user,
    [
        &BroadcasterId::from("1234"),
        &ModeratorId::from("5678"),
        &UserId::from("5432"),
    ]
);
api_test!(build
    get_unban_requests |api| {
        api.get_unban_requests(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("274637212"),
            UnbanRequestStatus::Pending,
        )
    }
);
api_test!(build
    resolve_unban_request |api| {
        api.resolve_unban_request(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("987654321"),
            "92af127c-7326-4483-a52b-b0daa0be61c01",
            UnbanRequestStatus::Approved,
        )
    }
);
api_test!(build
    get_blocked_terms |api| {
        api.get_blocked_terms(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .first(10)
    }
);
api_test!(
    add_blocked_term,
    [
        &BroadcasterId::from("1234"),
        &ModeratorId::from("5678"),
        "A phrase I’m not fond of",
    ]
);
api_test!(
    remove_blocked_term,
    [
        &BroadcasterId::from("1234"),
        &ModeratorId::from("5678"),
        &Id::from("c9fc79b8-0f63-4ef7-9d38-efd811e74ac2"),
    ]
);
api_test!(build
    delete_chat_messages |api| {
        api.delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
            .message_id("abc-123-def")
    }
);
api_test!(build
    get_moderated_channels |api| {
        api.get_moderated_channels(&UserId::from("931931"))
    }
);
api_test!(build
    get_moderators |api| {
        api.get_moderators(&BroadcasterId::from("198704263"))
    }
);
api_test!(
    add_channel_moderator,
    [&BroadcasterId::from("11111"), &UserId::from("44444"),]
);
api_test!(
    remove_channel_moderator,
    [&BroadcasterId::from("11111"), &UserId::from("44444"),]
);
api_test!(build
    get_vips |api| {
        api.get_vips(&BroadcasterId::from("123"))
    }
);
api_test!(
    add_channel_vip,
    [&BroadcasterId::from("123"), &UserId::from("456")]
);
api_test!(
    remove_channel_vip,
    [&BroadcasterId::from("123"), &UserId::from("456")]
);
api_test!(
    update_shield_mode_status,
    [
        &BroadcasterId::from("12345"),
        &ModeratorId::from("98765"),
        false,
    ]
);
api_test!(
    get_shield_mode_status,
    [&BroadcasterId::from("12345"), &ModeratorId::from("98765"),]
);
api_test!(
    warn_chat_user,
    [
        &BroadcasterId::from("404040"),
        &ModeratorId::from("404041"),
        &UserId::from("9876"),
        "stop doing that!"
    ]
);

#[tokio::test]
async fn ban_user2() {
    let suite = HttpMock::new().await;

    suite.ban_user2().await;

    let _ = suite
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
        .await
        .unwrap();
}

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

#[tokio::test]
async fn add_blocked_term2() {
    let suite = HttpMock::new().await;

    suite.add_blocked_term2().await;

    suite
        .execute(|api| {
            api.add_blocked_term(
                &BroadcasterId::from("1234"),
                &ModeratorId::from("5678"),
                "crac*",
            )
        })
        .json()
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_chat_messages2() {
    let suite = HttpMock::new().await;

    suite.delete_chat_messages2().await;

    suite
        .execute(|api| {
            api.delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
                .message_id("abc-123-def")
                .build()
        })
        .json()
        .await
        .unwrap();
}

#[tokio::test]
async fn get_vips2() {
    let suite = HttpMock::new().await;

    suite.get_vips2().await;

    suite
        .execute(|api| {
            api.get_vips(&BroadcasterId::from("123"))
                .user_ids(&[UserId::from("456"), UserId::from("678")])
                .build()
        })
        .json()
        .await
        .unwrap();
}

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let api = TwitchFixture::user_access_token(|scope| {
        scope
            .check_automod_status()
            .manage_held_automod_messages()
            .ban_user()
            .delete_chat_messages()
            .add_channel_moderator()
            .get_vips()
            .add_channel_vip()
            .update_shield_mode_status();
    })
    .await?;

    mock_api_check_automod_status(&api).await?;
    // mock_api_manage_held_automod_messages(&api).await?; // required msg_id
    // mock_api_get_automod_settings(&api).await?; // Not Found
    // mock_api_update_automod_settings(&api).await?; // Not Found

    mock_api_get_banned_users(&api).await?;

    mock_api_ban_user(&api).await?;
    // mock_api_unban_user(&api).await?; // user is not banned
    // mock_api_get_unban_requests(&api).await?; // Not Found
    // mock_api_resolve_unban_request(&api).await?; // Not Found
    // mock_api_get_blocked_terms(&api).await?; // Not Found
    // mock_api_add_blocked_term(&api).await?; // Not Found
    // mock_api_remove_blocked_term(&api).await?; // Not Found
    // mock_api_delete_chat_messages(&api).await?; // Missing required parameter message_id
    // mock_api_get_automod_settings(&api).await?; // Not Found
    // mock_api_get_moderated_channels(&api).await?; // Not Found
    mock_api_get_moderators(&api).await?;
    // mock_api_add_channel_moderator(&api).await?; // The user specified in parameter moderator_id
    // is already a moderator on this channel
    // mock_api_remove_channel_moderator(&api).await?; // The user specified in parameter moderator_id
    // is already a moderator on this channel
    mock_api_get_vips(&api).await?;
    mock_api_add_channel_vip(&api).await?;
    mock_api_remove_channel_vip(&api).await?;
    // mock_api_update_shield_mode_status(&api).await?; // User is already a VIP
    mock_api_get_shield_mode_status(&api).await?;
    // mock_api_warn_chat_user(&api).await?; // Not Found

    Ok(())
}

async fn mock_api_check_automod_status(api: &TwitchFixture) -> Result<()> {
    api.api
        .check_automod_status(&api.selected_broadcaster_id(), &[])
        .json()
        .await?;
    Ok(())
}

// async fn mock_api_manage_held_automod_messages(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .manage_held_automod_messages(&api.selected_user_id(), "", AutoModAction::ALLOW)
//         .json()
//         .await?;
//     Ok(())
// }
//
// async fn mock_api_get_automod_settings(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_automod_settings(&api.selected_broadcaster_id(), &api.selected_moderator_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_update_automod_settings(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .update_automod_settings(&api.selected_broadcaster_id(), &api.selected_moderator_id())
//         .json()
//         .await?;
//     Ok(())
// }
async fn mock_api_get_banned_users(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .get_banned_users(&api.selected_broadcaster_id())
        .json()
        .await?;

    Ok(())
}
async fn mock_api_ban_user(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let user_id = UserId::from(random_user.id);
    let _resp = api
        .api
        .ban_user(
            &api.selected_broadcaster_id(),
            &api.selected_moderator_id(),
            &user_id,
        )
        .send()
        .await?;

    Ok(())
}
// async fn mock_api_unban_user(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .unban_user(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             &api.selected_user_id(),
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_unban_requests(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_unban_requests(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             UnbanRequestStatus::Pending,
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_resolve_unban_request(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .resolve_unban_request(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             "",
//             UnbanRequestStatus::Pending,
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_blocked_terms(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_blocked_terms(&api.selected_broadcaster_id(), &api.selected_moderator_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_add_blocked_term(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .add_blocked_term(
//             &api.selected_broadcaster_id(),
//             &api.selected_broadcaster_id().to_moderator(),
//             "",
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_remove_blocked_term(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .add_blocked_term(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             "",
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_delete_chat_messages(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .delete_chat_messages(&api.selected_broadcaster_id(), &api.selected_moderator_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_moderated_channels(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_moderated_channels(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
async fn mock_api_get_moderators(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_moderators(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
// async fn mock_api_add_channel_moderator(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .add_channel_moderator(&api.selected_broadcaster_id(), &api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_remove_channel_moderator(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .remove_channel_moderator(&api.selected_broadcaster_id(), &api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
async fn mock_api_get_vips(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_vips(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_add_channel_vip(api: &TwitchFixture) -> Result<()> {
    api.api
        .add_channel_vip(&api.selected_broadcaster_id(), &api.selected_user_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_remove_channel_vip(api: &TwitchFixture) -> Result<()> {
    api.api
        .remove_channel_vip(&api.selected_broadcaster_id(), &api.selected_user_id())
        .json()
        .await?;
    Ok(())
}
// async fn mock_api_update_shield_mode_status(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .update_shield_mode_status(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             false,
//         )
//         .json()
//         .await?;
//     Ok(())
// }
async fn mock_api_get_shield_mode_status(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_shield_mode_status(&api.selected_broadcaster_id(), &api.selected_moderator_id())
        .json()
        .await?;
    Ok(())
}
// async fn mock_api_warn_chat_user(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .warn_chat_user(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             &api.selected_user_id(),
//             "",
//         )
//         .json()
//         .await?;
//     Ok(())
// }
