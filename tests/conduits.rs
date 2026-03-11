#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    conduits::ConduitsAPI,
    types::{ConduitId, ShardId},
};

#[tokio::test]
async fn get_conduits() {
    let suite = HttpMock::new().await;
    suite.get_conduits().await;

    let result = suite.api().get_conduits().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_conduits() {
    let suite = HttpMock::new().await;
    suite.create_conduits().await;

    let result = suite.api().create_conduits(5).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_conduits() {
    let suite = HttpMock::new().await;
    suite.update_conduits().await;

    let result = suite
        .api()
        .update_conduits(&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"), 5)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_conduits() {
    let suite = HttpMock::new().await;
    suite.delete_conduits().await;

    let result = suite
        .api()
        .delete_conduits(&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_conduit_shards() {
    let suite = HttpMock::new().await;
    suite.get_conduit_shards().await;

    let result = suite
        .api()
        .get_conduit_shards(&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_conduit_shards() {
    let suite = HttpMock::new().await;
    suite.update_conduit_shards().await;

    let result = suite
        .api()
        .update_conduit_shards(ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
        .webhook(
            ShardId::from("0"),
            "https://this-is-a-callback.com",
            "s3cre7",
        )
        .webhook(
            ShardId::from("1"),
            "https://this-is-a-callback-2.com",
            "s3cre7",
        )
        .webhook(
            ShardId::from("3"),
            "https://this-is-a-callback-3.com",
            "s3cre7",
        )
        .send()
        .await;

    assert!(result.is_ok());
}
