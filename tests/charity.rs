#![cfg(feature = "charity")]

#[macro_use]
mod common;

use twitch_highway::{charity::CharityAPI, types::BroadcasterId};

api_test!(get_charity_campaign[&BroadcasterId::from("123456")]);

api_test!(
    get_charity_campaign_donations | api | {
        api.get_charity_campaign_donations(&BroadcasterId::from("123456"))
    }
);
