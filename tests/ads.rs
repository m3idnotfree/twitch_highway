#[macro_use]
mod common;

use twitch_highway::{ads::AdsAPI, types::BroadcasterId};

api_test!(
    start_commercial[&BroadcasterId::from("141981764"), 60],
    get_ad_schedule[&BroadcasterId::from("123")],
    snooze_next_ad[&BroadcasterId::from("123")],
);
