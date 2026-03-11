#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    extensions::{ExtensionsAPI, Segment},
    types::{BroadcasterId, Cost, CostType, ExtensionId, JWTToken},
};

#[tokio::test]
async fn get_extension_configuration_segment() {
    let suite = HttpMock::new().await;
    suite.get_extension_configuration_segment().await;

    let result = suite
        .api()
        .get_extension_configuration_segment(
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn set_extension_configuration_segment() {
    let suite = HttpMock::new().await;
    suite.set_extension_configuration_segment().await;

    let result = suite
        .api()
        .set_extension_configuration_segment(
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Segment::Global,
        )
        .version("0.0.1")
        .content("hello config!")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn set_extension_required_configuration() {
    let suite = HttpMock::new().await;
    suite.set_extension_required_configuration().await;

    let result = suite
        .api()
        .set_extension_required_configuration(
            &BroadcasterId::from("274637212"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            "0.0.1",
            "RCS",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_extension_pubsub_message() {
    let suite = HttpMock::new().await;
    suite.send_extension_pubsub_message().await;

    let result = suite
        .api()
        .send_extension_pubsub_message(
            &["broadcast"],
            "hello world!",
            &BroadcasterId::from("141981764"),
            None,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extension_secrets() {
    let suite = HttpMock::new().await;
    suite.get_extension_secrets().await;

    let result = suite
        .api()
        .get_extension_secrets(&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_extension_secret() {
    let suite = HttpMock::new().await;
    suite.create_extension_secret().await;

    let result = suite
        .api()
        .create_extension_secret(
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some(600),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_extension_chat_message() {
    let suite = HttpMock::new().await;
    suite.send_extension_chat_message().await;

    let result = suite
        .api()
        .send_extension_chat_message(
            &BroadcasterId::from("237757755"),
            "Hello",
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            "0.0.9",
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extensions() {
    let suite = HttpMock::new().await;
    suite.get_extensions().await;

    let result = suite
        .api()
        .get_extensions(
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some("0.0.9"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_released_extensions() {
    let suite = HttpMock::new().await;
    suite.get_released_extensions().await;

    let result = suite
        .api()
        .get_released_extensions(
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some("0.0.9"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extension_bits_products() {
    let suite = HttpMock::new().await;
    suite.get_extension_bits_products().await;

    let result = suite.api().get_extension_bits_products(Some(true)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extension_live_channels() {
    let suite = HttpMock::new().await;
    suite.get_extension_live_channels().await;

    let result = suite
        .api()
        .get_extension_live_channels(&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_extension_bits_product() {
    let suite = HttpMock::new().await;
    suite.update_extension_bits_product().await;

    let result = suite
        .api()
        .update_extension_bits_product("1010", Cost::new(990, CostType::Bits), "Rusty Crate 2")
        .in_development(true)
        .is_broadcast(true)
        .expiration("2021-05-18T09:10:13.397Z")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extension_configuration_segment2() {
    let suite = HttpMock::new().await;
    suite.get_extension_configuration_segment2().await;

    let result = suite
        .api()
        .get_extension_configuration_segment(
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
        )
        .send()
        .await;

    assert!(result.is_ok());
}
