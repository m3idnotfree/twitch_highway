#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{ads::AdsAPI, types::BroadcasterId};

#[tokio::test]
async fn start_commercial() {
    let suite = HttpMock::new().await;
    suite.start_commercial().await;

    let result = suite
        .api()
        .start_commercial(&BroadcasterId::from("141981764"), 60)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_ad_schedule() {
    let suite = HttpMock::new().await;
    suite.get_ad_schedule().await;

    let result = suite
        .api()
        .get_ad_schedule(&BroadcasterId::from("123"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn snooze_next_ad() {
    let suite = HttpMock::new().await;
    suite.snooze_next_ad().await;

    let result = suite
        .api()
        .snooze_next_ad(&BroadcasterId::from("123"))
        .await;

    assert!(result.is_ok());
}
