#![cfg(feature = "subscriptions")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    subscriptions::SubscriptionsAPI,
    types::{BroadcasterId, UserId},
};
use twitch_oauth_token::scope::SubscriptionScopes;

api_test!(
    get_broadcaster_subscriptions,
    [&BroadcasterId::from("141981764"), None, None]
);
api_test!(
    check_user_subscription,
    [
        &BroadcasterId::from("149747285"),
        &UserId::from("141981764")
    ]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.subscriptions_api();
    })
    .await?;

    mock_api_get_broadcaster_subscriptions(&api).await?;
    mock_api_check_user_subscription(&api).await?;

    Ok(())
}

async fn mock_api_get_broadcaster_subscriptions(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_broadcaster_subscriptions(&api.selected_broadcaster_id(), None, None)
        .json()
        .await?;

    Ok(())
}
async fn mock_api_check_user_subscription(api: &TwitchFixture) -> Result<()> {
    api.api
        .check_user_subscription(&api.selected_broadcaster_id(), &api.selected_user_id())
        .json()
        .await?;

    Ok(())
}
