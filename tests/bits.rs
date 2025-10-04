#![cfg(feature = "bits")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    bits::{BitsAPI, BitsLeaderboardRequest},
    types::{BroadcasterId, ExtensionId},
};
use twitch_oauth_token::scope::BitScopes;

api_test!(
    get_bits_leaderboard,
    [Some(
        BitsLeaderboardRequest::new()
            .count(2)
            .period("week")
            .started_at(&"2018-02-05T08:00:00Z".parse().unwrap())
    )]
);
api_test!(get_cheermotes, [Some(&BroadcasterId::from("41245072"))]);
api_test!(
    get_extension_transactions,
    [&ExtensionId::from("1234"), None, None]
);

api_test!(extra
        get_cheermotes, 
        get_cheermotes2, 
        [Some(&BroadcasterId::from("41245072"))]);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.bits_api();
    })
    .await?;

    mock_api_get_bits_leaderboard(&api).await?;
    mock_api_get_cheermotes(&api).await?;

    Ok(())
}

async fn mock_api_get_bits_leaderboard(api: &TwitchFixture) -> Result<()> {
    let resp = api.api.get_bits_leaderboard(None).json().await?;
    assert!(!resp.data.is_empty());
    Ok(())
}

async fn mock_api_get_cheermotes(api: &TwitchFixture) -> Result<()> {
    let resp = api.api.get_cheermotes(None).json().await?;
    assert!(!resp.data.is_empty());
    Ok(())
}
