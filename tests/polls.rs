#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    polls::{EndPollStatus, PollsAPI},
    types::{BroadcasterId, PollId, Title},
};

#[tokio::test]
async fn get_polls() {
    let suite = HttpMock::new().await;
    suite.get_polls().await;

    let result = suite
        .api()
        .get_polls(&BroadcasterId::from("141981764"))
        .ids(&[PollId::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_poll() {
    let suite = HttpMock::new().await;
    suite.create_poll().await;

    let result = suite
        .api()
        .create_poll(
            &BroadcasterId::from("141981764"),
            "Heads or Tails?",
            &[Title::new("Heads"), Title::new("Tails")],
            1800,
        )
        .channel_points_voting_enabled(true)
        .channel_points_per_vote(100)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn end_poll() {
    let suite = HttpMock::new().await;
    suite.end_poll().await;

    let result = suite
        .api()
        .end_poll(
            &BroadcasterId::from("141981764"),
            &PollId::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"),
            EndPollStatus::TERMINATED,
        )
        .await;

    assert!(result.is_ok());
}
