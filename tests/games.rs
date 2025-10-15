#![cfg(feature = "games")]

#[macro_use]
mod common;

use twitch_highway::{games::GamesAPI, types::Id};

api_test!(build
    get_top_games |api| {
        api.get_top_games()
    }
);
api_test!(build
    get_games |api| {
        api.get_games().ids(&[Id::from("33214")])
    }
);
