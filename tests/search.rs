#![cfg(feature = "search")]

#[macro_use]
mod common;

use anyhow::Result;
use common::HttpMock;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::search::SearchAPI;
use twitch_oauth_token::scope::SearchScopes;

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

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.search_api();
    })
    .await?;

    mock_api_search_categories(&api).await?;
    mock_api_search_channels(&api).await?;

    Ok(())
}

async fn mock_api_search_categories(api: &TwitchFixture) -> Result<()> {
    api.api.search_categories("ff").json().await?;
    Ok(())
}
async fn mock_api_search_channels(api: &TwitchFixture) -> Result<()> {
    api.api.search_channels("ff").json().await?;
    Ok(())
}
