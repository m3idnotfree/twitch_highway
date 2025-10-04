#![cfg(feature = "channels")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    channels::{
        ChannelsAPI, ContentClassificationLabel, ContentClassificationLabelsID,
        ModifyChannelRequest,
    },
    types::{BroadcasterId, GameId, UserId},
};
use twitch_oauth_token::scope::ChannelScopes;

api_test!(get_channel_info, [&[BroadcasterId::from("141981764")]]);

api_test!(
    modify_channel_info,
    [
        &BroadcasterId::from("41245072"),
        ModifyChannelRequest::new()
            .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
            .game_id(&GameId::from("33214"))
            .broadcaster_language("en")
            .tags(&["LevelingUp"])
    ]
);

api_test!(get_channel_editors, [&BroadcasterId::from("141981764")]);

api_test!(get_followed_channels, [&UserId::from("123456"), None, None]);

api_test!(
    get_channel_followers,
    [&BroadcasterId::from("123456"), None, None,]
);

api_test!(extra
    modify_channel_info,
    modify_channel_info2,
    [
        &BroadcasterId::from("41245072"),
        ModifyChannelRequest::new()
            .game_id(&GameId::from("SomeGameID"))
                .content_classification_labels(&[
                    ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, true),
                    ContentClassificationLabel::new(ContentClassificationLabelsID::DrugsIntoxication, false)
            ])
            .is_branded_content(true)
    ]
);

api_test!(extra
    get_followed_channels,
    get_followed_channels2,
    [&UserId::from("123456"), Some(&BroadcasterId::from("654321")), None]
);

api_test!(extra
    get_channel_followers,
    get_channel_followers2,
    [
        &BroadcasterId::from("123456"),
        Some(&UserId::from("654321")),
        None,
    ]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.channel_api();
    })
    .await?;

    mock_api_get_channel_info(&api).await?;
    mock_api_modify_channel_info(&api).await?;
    mock_api_get_channel_editors(&api).await?;
    mock_api_get_followed_channels(&api).await?;
    mock_api_get_channel_followers(&api).await?;

    Ok(())
}

async fn mock_api_get_channel_info(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_info(&[api.selected_broadcaster_id()])
        .json()
        .await?;
    Ok(())
}
async fn mock_api_modify_channel_info(api: &TwitchFixture) -> Result<()> {
    api.api
        .modify_channel_info(
            &api.selected_broadcaster_id(),
            ModifyChannelRequest::new().broadcaster_language("en"),
        )
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_editors(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_editors(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_followed_channels(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_followed_channels(&api.selected_user_id(), None, None)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_followers(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_followers(&api.selected_broadcaster_id(), None, None)
        .json()
        .await?;
    Ok(())
}
