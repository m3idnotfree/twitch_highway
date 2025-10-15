#![cfg(feature = "channels")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    channels::{ChannelsAPI, ContentClassificationLabel, ContentClassificationLabelsID},
    types::{BroadcasterId, GameId, UserId},
};
use twitch_oauth_token::scope::ChannelScopes;

api_test!(get_channel_info, [&[BroadcasterId::from("141981764")]]);

api_test!(build
    modify_channel_info |api| {
        api.modify_channel_info(&BroadcasterId::from("41245072"))
            .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
            .game_id(&GameId::from("33214"))
            .broadcaster_language("en")
            .tags(&["LevelingUp"])
    }
);

api_test!(get_channel_editor, [&BroadcasterId::from("141981764")]);

api_test!(build
    get_followed_channels |api| {
        api.get_followed_channels(&UserId::from("123456"))
    }
);
api_test!(build
    get_channel_followers |api| {
        api.get_channel_followers(&BroadcasterId::from("123456"))
    }
);

api_test!(build_extra
    modify_channel_info,
    modify_channel_info2 |api| {
        api.modify_channel_info(&BroadcasterId::from("41245072"))
            .game_id(&GameId::from("SomeGameID"))
            .content_classification_labels(&[
                ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, true),
                ContentClassificationLabel::new(
                    ContentClassificationLabelsID::DrugsIntoxication,
                    false,
                ),
            ])
            .is_branded_content(true)
    }
);

api_test!(build_extra
    get_followed_channels,
    get_followed_channels2 |api| {
        api.get_followed_channels(&UserId::from("123456"))
            .broadcaster_id(&BroadcasterId::from("654321"))
    }
);

api_test!(build_extra
    get_channel_followers,
    get_channel_followers2 |api| {
        api.get_channel_followers(&BroadcasterId::from("123456"))
            .user_id(&UserId::from("654321"))
    }
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
        .modify_channel_info(&api.selected_broadcaster_id())
        .broadcaster_language("en")
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_editors(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_editor(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_followed_channels(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_followed_channels(&api.selected_user_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_followers(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_followers(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
