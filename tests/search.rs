#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::search::SearchAPI;

#[tokio::test]
async fn search_categories() {
    let suite = HttpMock::new().await;
    suite.search_categories().await;

    let result = suite.api().search_categories("fort").send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn search_channels() {
    let suite = HttpMock::new().await;
    suite.search_channels().await;

    let result = suite.api().search_channels("twitchdev").send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn search_channels_2() {
    let suite = HttpMock::new().await;
    suite.search_channels_2().await;

    let result = suite
        .api()
        .search_channels("a_seagull")
        .live_only(true)
        .send()
        .await;

    assert!(result.is_ok());
}
