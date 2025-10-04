#![cfg(feature = "ccls")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::ccls::CclsAPI;
use twitch_oauth_token::scope::CCLScopes;

api_test!(get_content_classification_labels, [None]);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.ccl_api();
    })
    .await?;

    let resp = api
        .api
        .get_content_classification_labels(None)
        .json()
        .await?;

    assert!(!resp.data.is_empty());

    Ok(())
}
