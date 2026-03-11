#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{raid::RaidAPI, types::BroadcasterId};

#[tokio::test]
async fn start_raid() {
    let suite = HttpMock::new().await;
    suite.start_raid().await;

    let result = suite
        .api()
        .start_raid(
            &BroadcasterId::from("12345678"),
            &BroadcasterId::from("12345678"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn cancel_raid() {
    let suite = HttpMock::new().await;
    suite.cancel_raid().await;

    let result = suite
        .api()
        .cancel_raid(&BroadcasterId::from("12345678"))
        .await;

    assert!(result.is_ok());
}
