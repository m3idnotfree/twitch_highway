mod builder;
mod response;
mod types;

pub use builder::{GetExtensionBuilder, GetGameAnalyticsBuilder};
pub use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};
pub use types::{AnalyticsType, ExtensionAnalytic, GameAnalytic};

use crate::Client;

pub trait AnalyticsAPI {
    /// Gets an analytics report for one or more extensions
    ///
    /// # Returns
    ///
    /// Returns a [`GetExtensionBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     analytics::{AnalyticsAPI, AnalyticsType},
    ///     types::ExtensionId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let extension_id = ExtensionId::from("72189");
    /// let response = api
    ///     .get_extension_analytics()
    ///     .extension_id(&extension_id)
    ///     .kind(AnalyticsType::OverviewV2)
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .first(50)
    ///     .after("eyJiI...")
    ///     .json()
    ///     .await?;
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn get_extension_analytics<'a>(&'a self) -> GetExtensionBuilder<'a>;

    /// Gets an analytics report for one or more games
    ///
    /// # Returns
    ///
    /// Returns a [`GetGameAnalyticsBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     analytics::{AnalyticsAPI, AnalyticsType},
    ///     types::GameId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let game_id = GameId::from("72189");
    /// let response = api
    ///     .get_game_analytics()
    ///     .game_id(&game_id)
    ///     .kind(AnalyticsType::OverviewV2)
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .first(50)
    ///     .after("eyJiI...")
    ///     .json()
    ///     .await?;
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn get_game_analytics<'a>(&'a self) -> GetGameAnalyticsBuilder<'a>;
}

impl AnalyticsAPI for Client {
    fn get_extension_analytics<'a>(&'a self) -> GetExtensionBuilder<'a> {
        GetExtensionBuilder::new(self)
    }
    fn get_game_analytics<'a>(&'a self) -> GetGameAnalyticsBuilder<'a> {
        GetGameAnalyticsBuilder::new(self)
    }
}
