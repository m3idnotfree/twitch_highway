mod builder;
mod response;

pub use builder::{GetGames, GetTopGames};
pub use response::GamesResponse;

use crate::Client;

pub trait GamesAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn get_top_games<'a>(&'a self) -> GetTopGames<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-games>
    fn get_games<'a>(&'a self) -> GetGames<'a>;
}

impl GamesAPI for Client {
    fn get_top_games<'a>(&'a self) -> GetTopGames<'a> {
        GetTopGames::new(self)
    }

    fn get_games<'a>(&'a self) -> GetGames<'a> {
        GetGames::new(self)
    }
}
