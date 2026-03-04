#[macro_use]
mod common;

use twitch_highway::{raid::RaidAPI, types::BroadcasterId};

api_test!(
    start_raid[&BroadcasterId::from("12345678"), &BroadcasterId::from("12345678")],
    cancel_raid[&BroadcasterId::from("12345678")]
);
