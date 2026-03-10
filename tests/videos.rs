#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{types::VideoId, videos::VideosAPI};

#[tokio::test]
async fn get_videos() {
    let suite = HttpMock::new().await;
    suite.get_videos().await;

    let result = suite
        .api()
        .get_videos(&[VideoId::from("335921245")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_videos() {
    let suite = HttpMock::new().await;
    suite.delete_videos().await;

    let result = suite
        .api()
        .delete_videos(&[VideoId::from("1234"), VideoId::from("9876")])
        .await;

    assert!(result.is_ok());
}
