#![cfg(feature = "clips")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use std::time::Duration;
use tokio::time::sleep;
use twitch_highway::{
    clips::ClipsAPI,
    types::{BroadcasterId, ClipId},
};
use twitch_oauth_token::scope::ClipScopes;

api_test!(build
    create_clip |api| {
        api.create_clip(&BroadcasterId::from("44322889"))
    }
);
api_test!(build
    get_clips |api| {
        api.get_clips_by_ids(&[ClipId::from("AwkwardHelplessSalamanderSwiftRage")])
    }
);
api_test!(build_extra
    get_clips,
    get_clips2 |api| {
        api.get_clips_by_broadcaster_id(&BroadcasterId::from("1234"))
            .first(5)
    }
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    sleep(Duration::from_millis(500)).await;

    let api = TwitchFixture::user_access_token_with_live(|scope| {
        scope.clips_api();
    })
    .await?;

    mock_api_create_clip(&api).await?;
    mock_api_get_clips(&api).await?;

    Ok(())
}

async fn mock_api_create_clip(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .create_clip(&api.selected_broadcaster_id())
        .send()
        .await?;

    Ok(())
}
async fn mock_api_get_clips(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .get_clips_by_broadcaster_id(&api.selected_broadcaster_id())
        .json()
        .await?;

    Ok(())
}
