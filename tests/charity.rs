#![cfg(feature = "charity")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{charity::CharityAPI, types::BroadcasterId};
use twitch_oauth_token::scope::CharityScopes;

api_test!(get_charity_campaign, [&BroadcasterId::from("123456")]);

api_test!(
    build
    get_charity_campaign_donations |api| {
        api.get_charity_campaign_donations(&BroadcasterId::from("123456"))
    }
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.charity_api();
    })
    .await?;

    mock_api_get_charity_campaign(&api).await?;
    mock_api_get_charity_campaign_donations(&api).await?;

    Ok(())
}

async fn mock_api_get_charity_campaign(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_charity_campaign(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_charity_campaign_donations(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_charity_campaign_donations(&api.selected_broadcaster_id())
        .json()
        .await?;

    Ok(())
}
