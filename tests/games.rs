#![cfg(feature = "games")]

#[macro_use]
mod common;

use twitch_highway::{
    games::{GamesAPI, GetGamesRequest},
    types::Id,
};

api_test!(get_top_games, [None]);
api_test!(
    get_games,
    [GetGamesRequest::new().ids(&[Id::from("33214")])]
);
