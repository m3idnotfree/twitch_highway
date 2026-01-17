#![cfg(feature = "teams")]

#[macro_use]
mod common;

use twitch_highway::{
    teams::TeamsAPI,
    types::{BroadcasterId, TeamId},
};

api_test!(get_channel_teams, [&BroadcasterId::from("96909659")]);
api_test!(extra
    get_teams_by_id,
    get_teams, [&TeamId::from("6358")]
);
