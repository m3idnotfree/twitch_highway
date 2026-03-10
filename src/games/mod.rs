mod builder;
mod response;

pub use builder::{GetGamesBuilder, GetTopGamesBuilder};
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_top_games()
    ///     .json()
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
    fn get_top_games<'a>(&'a self) -> GetTopGamesBuilder<'a>;

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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_games()
    ///     .json()
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
    fn get_games<'a>(&'a self) -> GetGamesBuilder<'a>;
}

impl GamesAPI for Client {
    fn get_top_games<'a>(&'a self) -> GetTopGamesBuilder<'a> {
        GetTopGamesBuilder::new(self)
    }
    fn get_games<'a>(&'a self) -> GetGamesBuilder<'a> {
        GetGamesBuilder::new(self)
    }
}
