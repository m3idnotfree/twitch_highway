#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    channels::{ChannelsAPI, ContentClassificationLabel, ContentClassificationLabelsID},
    types::{BroadcasterId, GameId, UserId},
};

#[tokio::test]
async fn get_channel_info() {
    let suite = HttpMock::new().await;
    suite.get_channel_info().await;

    let result = suite
        .api()
        .get_channel_info(&[BroadcasterId::from("141981764")])
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_editor() {
    let suite = HttpMock::new().await;
    suite.get_channel_editor().await;

    let result = suite
        .api()
        .get_channel_editor(&BroadcasterId::from("141981764"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn modify_channel_info() {
    let suite = HttpMock::new().await;
    suite.modify_channel_info().await;

    let result = suite
        .api()
        .modify_channel_info(&BroadcasterId::from("41245072"))
        .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
        .game_id(&GameId::from("33214"))
        .broadcaster_language("en")
        .tags(&["LevelingUp"])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_followed_channels() {
    let suite = HttpMock::new().await;
    suite.get_followed_channels().await;

    let result = suite
        .api()
        .get_followed_channels(&UserId::from("123456"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_followers() {
    let suite = HttpMock::new().await;
    suite.get_channel_followers().await;

    let result = suite
        .api()
        .get_channel_followers(&BroadcasterId::from("123456"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn modify_channel_info2() {
    let suite = HttpMock::new().await;
    suite.modify_channel_info2().await;

    let result = suite
        .api()
        .modify_channel_info(&BroadcasterId::from("41245072"))
        .game_id(&GameId::from("SomeGameID"))
        .content_classification_labels(&[
            ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, true),
            ContentClassificationLabel::new(
                ContentClassificationLabelsID::DrugsIntoxication,
                false,
            ),
        ])
        .is_branded_content(true)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_followed_channels2() {
    let suite = HttpMock::new().await;
    suite.get_followed_channels2().await;

    let result = suite
        .api()
        .get_followed_channels(&UserId::from("123456"))
        .broadcaster_id(&BroadcasterId::from("654321"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_followers2() {
    let suite = HttpMock::new().await;
    suite.get_channel_followers2().await;

    let result = suite
        .api()
        .get_channel_followers(&BroadcasterId::from("123456"))
        .user_id(&UserId::from("654321"))
        .send()
        .await;

    assert!(result.is_ok());
}
