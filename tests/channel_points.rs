#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    channel_points::{ChannelPointsAPI, RedemptionStatus},
    types::{BroadcasterId, RedemptionId, RewardId},
};

#[tokio::test]
async fn create_custom_rewards() {
    let suite = HttpMock::new().await;
    suite.create_custom_rewards().await;

    let result = suite
        .api()
        .create_custom_rewards(
            &BroadcasterId::from("274637212"),
            "game analysis 1v1",
            50000,
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_custom_reward() {
    let suite = HttpMock::new().await;
    suite.delete_custom_reward().await;

    let result = suite
        .api()
        .delete_custom_reward(
            &BroadcasterId::from("274637212"),
            &RewardId::from("b045196d-9ce7-4a27-a9b9-279ed341ab28"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_custom_reward() {
    let suite = HttpMock::new().await;
    suite.get_custom_reward().await;

    let result = suite
        .api()
        .get_custom_reward(&BroadcasterId::from("274637212"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_custom_reward_redemption() {
    let suite = HttpMock::new().await;
    suite.get_custom_reward_redemption().await;

    let result = suite
        .api()
        .get_custom_reward_redemption(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .status(RedemptionStatus::CANCELED)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_custom_reward() {
    let suite = HttpMock::new().await;
    suite.update_custom_reward().await;

    let result = suite
        .api()
        .update_custom_reward(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .is_enabled(false)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_redemption_status() {
    let suite = HttpMock::new().await;
    suite.update_redemption_status().await;

    let result = suite
        .api()
        .update_redemption_status(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
            &[RedemptionId::from("17fa2df1-ad76-4804-bfa5-a40ef63efe63")],
            RedemptionStatus::CANCELED,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_custom_reward2() {
    let suite = HttpMock::new().await;
    suite.get_custom_reward2().await;

    let result = suite
        .api()
        .get_custom_reward(&BroadcasterId::from("274637212"))
        .only_manageable_rewards(true)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_custom_reward3() {
    let suite = HttpMock::new().await;
    suite.get_custom_reward3().await;

    let result = suite
        .api()
        .get_custom_reward(&BroadcasterId::from("274637212"))
        .custom_reward_ids(&[RewardId::from("2af127c-7326-4483-a52b-b0da0be61c01")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_custom_reward_redemption2() {
    let suite = HttpMock::new().await;
    suite.get_custom_reward_redemption2().await;

    let result = suite
        .api()
        .get_custom_reward_redemption(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_custom_reward2() {
    let suite = HttpMock::new().await;
    suite.update_custom_reward2().await;

    let result = suite
        .api()
        .update_custom_reward(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .title("game analysis 2v2")
        .send()
        .await;

    assert!(result.is_ok());
}
