#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{analytics::AnalyticsAPI, types::GameId};

#[tokio::test]
async fn get_extension_analytics() {
    let suite = HttpMock::new().await;
    suite.get_extension_analytics().await;

    let result = suite.api().get_extension_analytics().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_game_analytics() {
    let suite = HttpMock::new().await;
    suite.get_game_analytics().await;

    let result = suite
        .api()
        .get_game_analytics()
        .game_id(&GameId::from("493057"))
        .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
        .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_game_analytics2() {
    let suite = HttpMock::new().await;
    suite.get_game_analytics2().await;

    let result = suite.api().get_game_analytics().first(5).send().await;

    assert!(result.is_ok());
}
