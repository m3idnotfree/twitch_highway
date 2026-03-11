#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{charity::CharityAPI, types::BroadcasterId};

#[tokio::test]
async fn get_charity_campaign() {
    let suite = HttpMock::new().await;
    suite.get_charity_campaign().await;

    let result = suite
        .api()
        .get_charity_campaign(&BroadcasterId::from("123456"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_charity_campaign_donations() {
    let suite = HttpMock::new().await;
    suite.get_charity_campaign_donations().await;

    let result = suite
        .api()
        .get_charity_campaign_donations(&BroadcasterId::from("123456"))
        .send()
        .await;

    assert!(result.is_ok());
}
