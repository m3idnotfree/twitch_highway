#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{games::GamesAPI, types::GameId};

#[tokio::test]
async fn get_top_games() {
    let suite = HttpMock::new().await;
    suite.get_top_games().await;

    let result = suite.api().get_top_games().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_games() {
    let suite = HttpMock::new().await;
    suite.get_games().await;

    let result = suite
        .api()
        .get_games()
        .ids(&[GameId::from("33214")])
        .send()
        .await;

    assert!(result.is_ok());
}
