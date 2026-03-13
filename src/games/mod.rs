mod builder;
mod response;

pub use builder::{GetGames, GetTopGames};
pub use response::GamesResponse;

use crate::Client;

pub trait GamesAPI {
    /// Gets information about all broadcasts on Twitch
    ///
    /// # Returns
    ///
    /// Returns a [`GetTopGamesBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::games::GamesAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_top_games()
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn get_top_games<'a>(&'a self) -> GetTopGames<'a>;

    /// Gets information about specified categories or games
    ///
    /// # Returns
    ///
    /// Returns a [`GetGamesBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::games::GamesAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_games()
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
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
