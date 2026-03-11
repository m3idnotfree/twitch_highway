#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{hype_train::HypeTrainAPI, types::BroadcasterId};

#[tokio::test]
async fn get_hype_train_status() {
    let suite = HttpMock::new().await;
    suite.get_hype_train_status().await;

    let result = suite
        .api()
        .get_hype_train_status(&BroadcasterId::from("123"))
        .await;

    assert!(result.is_ok());
}
