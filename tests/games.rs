#![cfg(feature = "games")]

#[macro_use]
mod common;

use twitch_highway::{games::GamesAPI, types::GameId};

api_test!(get_top_games | api | { api.get_top_games() });

api_test!(get_games | api | { api.get_games().ids(&[GameId::from("33214")]) });
