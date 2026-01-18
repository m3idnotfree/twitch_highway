#![cfg(feature = "goals")]

#[macro_use]
mod common;

use twitch_highway::{goals::GoalsAPI, types::BroadcasterId};

api_test!(get_creator_goals[&BroadcasterId::from("141981764")]);
