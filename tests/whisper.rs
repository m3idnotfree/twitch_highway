#![cfg(feature = "whisper")]

#[macro_use]
mod common;

use twitch_highway::{types::UserId, whisper::WhisperAPI};

api_test!(
    send_whisper,
    [&UserId::from("123"), &UserId::from("456"), "hello"]
);
