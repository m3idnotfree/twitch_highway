#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    guest_star::{GroupLayout, GuestStarAPI},
    types::{BroadcasterId, ModeratorId, SessionId, UserId},
};

#[tokio::test]
async fn get_channel_guest_star_settings() {
    let suite = HttpMock::new().await;
    suite.get_channel_guest_star_settings().await;

    let result = suite
        .api()
        .get_channel_guest_star_settings(
            &BroadcasterId::from("932104"),
            &ModeratorId::from("9321049"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_guest_star_session() {
    let suite = HttpMock::new().await;
    suite.get_guest_star_session().await;

    let result = suite
        .api()
        .get_guest_star_session(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_guest_star_session() {
    let suite = HttpMock::new().await;
    suite.create_guest_star_session().await;

    let result = suite
        .api()
        .create_guest_star_session(&BroadcasterId::from("9321049"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn end_guest_star_session() {
    let suite = HttpMock::new().await;
    suite.end_guest_star_session().await;

    let result = suite
        .api()
        .end_guest_star_session(
            &BroadcasterId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_guest_star_invites() {
    let suite = HttpMock::new().await;
    suite.get_guest_star_invites().await;

    let result = suite
        .api()
        .get_guest_star_invites(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_guest_star_invite() {
    let suite = HttpMock::new().await;
    suite.send_guest_star_invite().await;

    let result = suite
        .api()
        .send_guest_star_invite(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            &UserId::from("144601104"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_guest_star_invite() {
    let suite = HttpMock::new().await;
    suite.delete_guest_star_invite().await;

    let result = suite
        .api()
        .delete_guest_star_invite(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            &UserId::from("144601104"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn assign_guest_star_slot() {
    let suite = HttpMock::new().await;
    suite.assign_guest_star_slot().await;

    let result = suite
        .api()
        .assign_guest_star_slot(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            &UserId::from("144601104"),
            "1",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_guest_star_settings() {
    let suite = HttpMock::new().await;
    suite.update_channel_guest_star_settings().await;

    let result = suite
        .api()
        .update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
        .group_layout(GroupLayout::TILED_LAYOUT)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_guest_star_slot() {
    let suite = HttpMock::new().await;
    suite.update_guest_star_slot().await;

    let result = suite
        .api()
        .update_guest_star_slot(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .destination_slot_id("2")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_guest_star_slot() {
    let suite = HttpMock::new().await;
    suite.delete_guest_star_slot().await;

    let result = suite
        .api()
        .delete_guest_star_slot(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            &UserId::from("144601104"),
            "1",
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_guest_star_slot_settings() {
    let suite = HttpMock::new().await;
    suite.update_guest_star_slot_settings().await;

    let result = suite
        .api()
        .update_guest_star_slot_settings(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .is_audio_enabled(false)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_guest_star_settings2() {
    let suite = HttpMock::new().await;
    suite.update_channel_guest_star_settings2().await;

    let result = suite
        .api()
        .update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
        .is_moderator_send_live_enabled(false)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_guest_star_settings3() {
    let suite = HttpMock::new().await;
    suite.update_channel_guest_star_settings3().await;

    let result = suite
        .api()
        .update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
        .slot_count(6)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_guest_star_settings4() {
    let suite = HttpMock::new().await;
    suite.update_channel_guest_star_settings4().await;

    let result = suite
        .api()
        .update_channel_guest_star_settings(&BroadcasterId::from("9321049"))
        .regenerate_browser_sources(true)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_guest_star_slot_settings2() {
    let suite = HttpMock::new().await;
    suite.update_guest_star_slot_settings2().await;

    let result = suite
        .api()
        .update_guest_star_slot_settings(
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            &SessionId::from("2KFRQbFtpmfyD3IevNRnCzOPRJI"),
            "1",
        )
        .is_live(true)
        .send()
        .await;

    assert!(result.is_ok());
}
