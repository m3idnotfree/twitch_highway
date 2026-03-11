#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    types::{BroadcasterId, ExtensionId, UserId},
    users::{Component, Overlay, Panel, UserAPI},
};

#[tokio::test]
async fn update_user() {
    let suite = HttpMock::new().await;
    suite.update_user().await;

    let result = suite.api().update_user("BaldAngel").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn unblock_user() {
    let suite = HttpMock::new().await;
    suite.unblock_user().await;

    let result = suite.api().unblock_user(&UserId::from("198704263")).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_extensions() {
    let suite = HttpMock::new().await;
    suite.get_user_extensions().await;

    let result = suite.api().get_user_extensions().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_authorization_by_user() {
    let suite = HttpMock::new().await;
    suite.get_authorization_by_user().await;

    let result = suite
        .api()
        .get_authorization_by_user(&[UserId::from("41981764"), UserId::from("197886470")])
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_users() {
    let suite = HttpMock::new().await;
    suite.get_users().await;

    let result = suite
        .api()
        .get_users()
        .ids(&[UserId::from("141981764")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_block_list() {
    let suite = HttpMock::new().await;
    suite.get_user_block_list().await;

    let result = suite
        .api()
        .get_user_block_list(&BroadcasterId::from("141981764"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn block_user() {
    let suite = HttpMock::new().await;
    suite.block_user().await;

    let result = suite
        .api()
        .block_user(&UserId::from("198704263"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_active_extensions() {
    let suite = HttpMock::new().await;
    suite.get_user_active_extensions().await;

    let result = suite.api().get_user_active_extensions().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_user_extensions() {
    let suite = HttpMock::new().await;
    suite.update_user_extensions().await;

    let result = suite
        .api()
        .update_user_extensions()
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
        .send()
        .await;

    assert!(result.is_ok());
}
