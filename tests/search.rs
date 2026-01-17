#![cfg(feature = "search")]

#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::search::SearchAPI;

api_test!(build
    search_categories |api| {
        api.search_categories("fort")
    }
);
api_test!(build
    search_channels |api| {
        api.search_channels("twitchdev")
    }
);

#[tokio::test]
pub(crate) async fn search_channels_2() {
    let suite = HttpMock::new().await;

    suite.search_channels_2().await;

    let _ = suite
        .execute(|api| api.search_channels("a_seagull").live_only(true).build())
        .json()
        .await
        .unwrap();
}
