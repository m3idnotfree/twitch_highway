#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    bits::{BitsAPI, Period},
    types::{BroadcasterId, ExtensionId},
};

#[tokio::test]
async fn get_bits_leaderboard() {
    let suite = HttpMock::new().await;
    suite.get_bits_leaderboard().await;

    let result = suite
        .api()
        .get_bits_leaderboard()
        .count(2)
        .period(Period::Week)
        .started_at(&"2018-02-05T08:00:00Z".parse().unwrap())
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_cheermotes() {
    let suite = HttpMock::new().await;
    suite.get_cheermotes().await;

    let result = suite
        .api()
        .get_cheermotes(Some(&BroadcasterId::from("41245072")))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_cheermotes2() {
    let suite = HttpMock::new().await;
    suite.get_cheermotes2().await;

    let result = suite
        .api()
        .get_cheermotes(Some(&BroadcasterId::from("41245072")))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_extension_transactions() {
    let suite = HttpMock::new().await;
    suite.get_extension_transactions().await;

    let result = suite
        .api()
        .get_extension_transactions(&ExtensionId::from("1234"))
        .send()
        .await;

    assert!(result.is_ok());
}
