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

#[tokio::test]
async fn check_automod_status() {
    let suite = HttpMock::new().await;
    suite.check_automod_status().await;

    let result = suite
        .api()
        .check_automod_status(
            &BroadcasterId::from("12345"),
            &[
                CheckAutoMod::new("123", "Hello World!"),
                CheckAutoMod::new("393", "Boooooo!"),
            ],
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn manage_held_automod_messages() {
    let suite = HttpMock::new().await;
    suite.manage_held_automod_messages().await;

    let result = suite
        .api()
        .manage_held_automod_messages(&UserId::from("9327994"), "836013710", AutoModAction::ALLOW)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_automod_settings() {
    let suite = HttpMock::new().await;
    suite.get_automod_settings().await;

    let result = suite
        .api()
        .get_automod_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn unban_user() {
    let suite = HttpMock::new().await;
    suite.unban_user().await;

    let result = suite
        .api()
        .unban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("5432"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_blocked_term() {
    let suite = HttpMock::new().await;
    suite.add_blocked_term().await;

    let result = suite
        .api()
        .add_blocked_term(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            "A phrase I’m not fond of",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_blocked_term() {
    let suite = HttpMock::new().await;
    suite.remove_blocked_term().await;

    let result = suite
        .api()
        .remove_blocked_term(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &BlockedTermId::from("c9fc79b8-0f63-4ef7-9d38-efd811e74ac2"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_channel_moderator() {
    let suite = HttpMock::new().await;
    suite.add_channel_moderator().await;

    let result = suite
        .api()
        .add_channel_moderator(&BroadcasterId::from("11111"), &UserId::from("44444"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_channel_moderator() {
    let suite = HttpMock::new().await;
    suite.remove_channel_moderator().await;

    let result = suite
        .api()
        .remove_channel_moderator(&BroadcasterId::from("11111"), &UserId::from("44444"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_channel_vip() {
    let suite = HttpMock::new().await;
    suite.add_channel_vip().await;

    let result = suite
        .api()
        .add_channel_vip(&BroadcasterId::from("123"), &UserId::from("456"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_channel_vip() {
    let suite = HttpMock::new().await;
    suite.remove_channel_vip().await;

    let result = suite
        .api()
        .remove_channel_vip(&BroadcasterId::from("123"), &UserId::from("456"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_shield_mode_status() {
    let suite = HttpMock::new().await;
    suite.update_shield_mode_status().await;

    let result = suite
        .api()
        .update_shield_mode_status(
            &BroadcasterId::from("12345"),
            &ModeratorId::from("98765"),
            false,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_shield_mode_status() {
    let suite = HttpMock::new().await;
    suite.get_shield_mode_status().await;

    let result = suite
        .api()
        .get_shield_mode_status(&BroadcasterId::from("12345"), &ModeratorId::from("98765"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn warn_chat_user() {
    let suite = HttpMock::new().await;
    suite.warn_chat_user().await;

    let result = suite
        .api()
        .warn_chat_user(
            &BroadcasterId::from("404040"),
            &ModeratorId::from("404041"),
            &UserId::from("9876"),
            "stop doing that!",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_blocked_term2() {
    let suite = HttpMock::new().await;
    suite.add_blocked_term2().await;

    let result = suite
        .api()
        .add_blocked_term(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            "crac*",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_automod_settings() {
    let suite = HttpMock::new().await;
    suite.update_automod_settings().await;

    let result = suite
        .api()
        .update_automod_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
        .overall_level(3)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_banned_users() {
    let suite = HttpMock::new().await;
    suite.get_banned_users().await;

    let result = suite
        .api()
        .get_banned_users(&BroadcasterId::from("198704263"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn ban_user() {
    let suite = HttpMock::new().await;
    suite.ban_user().await;

    let result = suite
        .api()
        .ban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("9876"),
        )
        .reason("no reason")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_unban_requests() {
    let suite = HttpMock::new().await;
    suite.get_unban_requests().await;

    let result = suite
        .api()
        .get_unban_requests(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("274637212"),
            UnbanRequestStatus::Pending,
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn resolve_unban_request() {
    let suite = HttpMock::new().await;
    suite.resolve_unban_request().await;

    let result = suite
        .api()
        .resolve_unban_request(
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("987654321"),
            "92af127c-7326-4483-a52b-b0daa0be61c01",
            UnbanRequestStatus::Approved,
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_blocked_terms() {
    let suite = HttpMock::new().await;
    suite.get_blocked_terms().await;

    let result = suite
        .api()
        .get_blocked_terms(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
        .first(10)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_chat_messages() {
    let suite = HttpMock::new().await;
    suite.delete_chat_messages().await;

    let result = suite
        .api()
        .delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
        .message_id("abc-123-def")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_moderated_channels() {
    let suite = HttpMock::new().await;
    suite.get_moderated_channels().await;

    let result = suite
        .api()
        .get_moderated_channels(&UserId::from("931931"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_moderators() {
    let suite = HttpMock::new().await;
    suite.get_moderators().await;

    let result = suite
        .api()
        .get_moderators(&BroadcasterId::from("198704263"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_vips() {
    let suite = HttpMock::new().await;
    suite.get_vips().await;

    let result = suite
        .api()
        .get_vips(&BroadcasterId::from("123"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn ban_user2() {
    let suite = HttpMock::new().await;
    suite.ban_user2().await;

    let result = suite
        .api()
        .ban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("9876"),
        )
        .duration(300)
        .reason("no reason")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn ban_user_error() {
    let suite = HttpMock::new().await;

    suite.ban_user_error().await;

    let respnose = suite
        .api()
        .ban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("9876"),
        )
        .duration(300)
        .reason("no reason")
        .send()
        .await;

    assert!(respnose.is_err());
}

#[tokio::test]
async fn unban_user_error() {
    let suite = HttpMock::new().await;

    suite.unban_user_error().await;

    let respnose = suite
        .api()
        .unban_user(
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("5432"),
        )
        .await;

    assert!(respnose.is_err());
}

#[tokio::test]
async fn delete_chat_messages2() {
    let suite = HttpMock::new().await;
    suite.delete_chat_messages2().await;

    let result = suite
        .api()
        .delete_chat_messages(&BroadcasterId::from("11111"), &ModeratorId::from("44444"))
        .message_id("abc-123-def")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_vips2() {
    let suite = HttpMock::new().await;
    suite.get_vips2().await;

    let result = suite
        .api()
        .get_vips(&BroadcasterId::from("123"))
        .user_ids(&[UserId::from("456"), UserId::from("678")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_suspicious_status_to_chat_user() {
    let suite = HttpMock::new().await;
    suite.add_suspicious_status_to_chat_user().await;

    let result = suite
        .api()
        .add_suspicious_status_to_chat_user(
            &BroadcasterId::from("141981764"),
            &ModeratorId::from("12826"),
            &UserId::from("9876"),
            SuspiciousStatus::Restricted,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_suspicious_status_from_chat_user() {
    let suite = HttpMock::new().await;
    suite.remove_suspicious_status_from_chat_user().await;

    let result = suite
        .api()
        .remove_suspicious_status_from_chat_user(
            &BroadcasterId::from("141981764"),
            &ModeratorId::from("12826"),
            &UserId::from("9876"),
        )
        .await;

    assert!(result.is_ok());
}
