#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    chat::{AnnouncementColor, ChatAPI, ChatColor},
    types::{BroadcasterId, ModeratorId, UserId},
};

#[tokio::test]
async fn get_chatters() {
    let suite = HttpMock::new().await;
    suite.get_chatters().await;

    let result = suite
        .api()
        .get_chatters(&BroadcasterId::from("123456"), &ModeratorId::from("654321"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_emotes() {
    let suite = HttpMock::new().await;
    suite.get_channel_emotes().await;

    let result = suite
        .api()
        .get_channel_emotes(&BroadcasterId::from("141981764"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_global_emotes() {
    let suite = HttpMock::new().await;
    suite.get_global_emotes().await;

    let result = suite.api().get_global_emotes().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_emote_sets() {
    let suite = HttpMock::new().await;
    suite.get_emote_sets().await;

    let result = suite.api().get_emote_sets(&["301590448"]).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_chat_badges() {
    let suite = HttpMock::new().await;
    suite.get_channel_chat_badges().await;

    let result = suite
        .api()
        .get_channel_chat_badges(&BroadcasterId::from("135093069"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_global_chat_badges() {
    let suite = HttpMock::new().await;
    suite.get_global_chat_badges().await;

    let result = suite.api().get_global_chat_badges().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_shared_chat_session() {
    let suite = HttpMock::new().await;
    suite.get_shared_chat_session().await;

    let result = suite
        .api()
        .get_shared_chat_session(&BroadcasterId::from("198704263"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_a_shoutout() {
    let suite = HttpMock::new().await;
    suite.send_a_shoutout().await;

    let result = suite
        .api()
        .send_a_shoutout(
            &BroadcasterId::from("12345"),
            &BroadcasterId::from("626262"),
            &ModeratorId::from("98765"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_chat_color() {
    let suite = HttpMock::new().await;
    suite.get_user_chat_color().await;

    let result = suite
        .api()
        .get_user_chat_color(&[UserId::from("11111"), UserId::from("44444")])
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_user_chat_color() {
    let suite = HttpMock::new().await;
    suite.update_user_chat_color().await;

    let result = suite
        .api()
        .update_user_chat_color(&UserId::from("123"), ChatColor::Blue)
        .await;

    assert!(result.is_ok());
}

// #[tokio::test]
// async fn update_user_chat_color2() {
//     let suite = HttpMock::new().await;
//     suite.update_user_chat_color2().await;
//
//     let result = suite
//         .api()
//         .update_user_chat_color(&UserId::from("123"), ChatColor::Blue)
//         .await;
//
//     assert!(result.is_ok());
// }

#[tokio::test]
async fn get_chat_settings() {
    let suite = HttpMock::new().await;
    suite.get_chat_settings().await;

    let result = suite
        .api()
        .get_chat_settings(&BroadcasterId::from("1234"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_emotes() {
    let suite = HttpMock::new().await;
    suite.get_user_emotes().await;

    let result = suite
        .api()
        .get_user_emotes(&UserId::from("123456"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_chat_settings() {
    let suite = HttpMock::new().await;
    suite.update_chat_settings().await;

    let result = suite
        .api()
        .update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
        .follower_mode(false)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_chat_announcement() {
    let suite = HttpMock::new().await;
    suite.send_chat_announcement().await;

    let result = suite
        .api()
        .send_chat_announcement(
            &BroadcasterId::from("11111"),
            &ModeratorId::from("44444"),
            "Hello chat!",
        )
        .color(AnnouncementColor::Purple)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_chat_message() {
    let suite = HttpMock::new().await;
    suite.send_chat_message().await;

    let result = suite
        .api()
        .send_chat_message(
            &BroadcasterId::from("12826"),
            &UserId::from("141981764"),
            "Hello, world! twitchdevHype",
        )
        .for_source_only(true)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_chat_settings2() {
    let suite = HttpMock::new().await;
    suite.update_chat_settings2().await;

    let result = suite
        .api()
        .update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
        .slow_mode(true)
        .slow_mode_wait_time(10)
        .send()
        .await;

    assert!(result.is_ok());
}
