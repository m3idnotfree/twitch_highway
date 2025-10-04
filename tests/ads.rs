#![cfg(feature = "ads")]

#[macro_use]
mod common;

use twitch_highway::{ads::AdsAPI, types::BroadcasterId};

api_test!(start_commercial, [&BroadcasterId::from("141981764"), 60]);
api_test!(get_ad_schedule, [&BroadcasterId::from("123")]);
api_test!(snooze_next_ad, [&BroadcasterId::from("123")]);
