#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{goals::GoalsAPI, types::BroadcasterId};

#[tokio::test]
async fn get_creator_goals() {
    let suite = HttpMock::new().await;
    suite.get_creator_goals().await;

    let result = suite
        .api()
        .get_creator_goals(&BroadcasterId::from("141981764"))
        .await;

    assert!(result.is_ok());
}
