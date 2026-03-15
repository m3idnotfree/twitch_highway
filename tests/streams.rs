#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
mod common;

use anyhow::Result;
use common::{HttpMock, TwitchFixture, mock_api_start};
use twitch_highway::{
    streams::StreamsAPI,
    types::{BroadcasterId, UserId, VideoId},
};

#[tokio::test]
async fn get_stream_key() {
    let suite = HttpMock::new().await;
    suite.get_stream_key().await;

    let result = suite
        .api()
        .get_stream_key(&BroadcasterId::from("141981764"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_stream_marker() {
    let suite = HttpMock::new().await;
    suite.create_stream_marker().await;

    let result = suite
        .api()
        .create_stream_marker(&UserId::from("123"), Some("hello, this is a marker!"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_streams() {
    let suite = HttpMock::new().await;
    suite.get_streams().await;

    let result = suite.api().get_streams().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_followed_streams() {
    let suite = HttpMock::new().await;
    suite.get_followed_streams().await;

    let result = suite
        .api()
        .get_followed_streams(&UserId::from("141981764"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_stream_markers_by_user_id() {
    let suite = HttpMock::new().await;
    suite.get_stream_markers().await;

    let result = suite
        .api()
        .get_stream_markers(&UserId::from("123"))
        .first(5)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_streams_2() {
    let suite = HttpMock::new().await;
    suite.get_streams_2().await;

    let result = suite
        .api()
        .get_streams()
        .user_logins(&["cohhcarnage", "lana_lux"])
        .send()
        .await;

    assert!(result.is_ok());
}

// #[tokio::test]
// async fn mock_api() -> Result<()> {
//     let _cmd = mock_api_start().await?;
//
//     let api = TwitchFixture::user_access_token_with_live(|scope| {
//         scope.streams_api();
//     })
//     .await?;
//
//     mock_api_get_stream_key(&api).await?;
//     mock_api_get_streams(&api).await?;
//     mock_api_get_followed_streams(&api).await?;
//
//     mock_api_create_stream_marker(&api).await?;
//
//     mock_api_get_stream_markers(&api).await?;
//     mock_api_get_stream_key(&api).await?;
//
//     Ok(())
// }
//
// async fn mock_api_get_stream_key(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_stream_key(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_streams(api: &TwitchFixture) -> Result<()> {
//     api.api.get_streams().json().await?;
//     Ok(())
// }
// async fn mock_api_get_followed_streams(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_followed_streams(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_create_stream_marker(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .create_stream_marker(&api.selected_user_id(), None)
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_stream_markers(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_stream_markers_by_user_id(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
