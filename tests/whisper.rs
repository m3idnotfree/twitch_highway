#![cfg(feature = "whisper")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{types::UserId, whisper::WhisperAPI};
use twitch_oauth_token::scope::WhisperScopes;

api_test!(
    send_whisper,
    [&UserId::from("123"), &UserId::from("456"), "hello"]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.whisper_api();
    })
    .await?;

    mock_api_send_whisper(&api).await?;

    Ok(())
}

async fn mock_api_send_whisper(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let moderation = UserId::from(api.selected_user.id.clone());

    let user = UserId::from(random_user.id);
    api.api
        .send_whisper(&moderation, &user, "hello")
        .send()
        .await?;

    Ok(())
}
