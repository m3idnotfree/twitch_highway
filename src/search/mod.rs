mod builder;
mod response;
mod types;

pub use builder::{SearchCategoriesBuilder, SearchChannelsBuilder};
pub use response::{CategoriesResponse, ChannelsResponse};
pub use types::Channel;

use crate::Client;

pub trait SearchAPI {
    /// Gets the games or categories that match the specified query
    ///
    /// # Arguments
    ///
    /// * `query` - The URI-encoded search string.
    ///
    /// # Returns
    ///
    /// Returns a [`SearchCategoriesBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::search::SearchAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .search_categories("%23archery")
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
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories<'a>(&'a self, query: &'a str) -> SearchCategoriesBuilder<'a>;

    /// Gets the channels that match the specified query and have streamed content within the past 6 months
    ///
    /// # Arguments
    ///
    /// * `query` - The URI-encoded search string.
    ///
    /// # Returns
    ///
    /// Returns a [`SearchChannelsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::search::SearchAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .search_channels("%20death")
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
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn search_channels<'a>(&'a self, query: &'a str) -> SearchChannelsBuilder<'a>;
}

impl SearchAPI for Client {
    fn search_categories<'a>(&'a self, query: &'a str) -> SearchCategoriesBuilder<'a> {
        SearchCategoriesBuilder::new(self, query)
    }
    fn search_channels<'a>(&'a self, query: &'a str) -> SearchChannelsBuilder<'a> {
        SearchChannelsBuilder::new(self, query)
    }
}
