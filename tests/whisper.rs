#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{types::UserId, whisper::WhisperAPI};

#[tokio::test]
async fn send_whisper() {
    let suite = HttpMock::new().await;
    suite.send_whisper().await;

    let result = suite
        .api()
        .send_whisper(&UserId::from("123"), &UserId::from("456"), "hello")
        .await;

    assert!(result.is_ok());
}
