mod builder;
mod response;
mod types;

pub use builder::{SearchCategories, SearchChannels};
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .search_categories("%23archery")
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
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories<'a>(&'a self, query: &'a str) -> SearchCategories<'a>;

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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .search_channels("%20death")
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
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn search_channels<'a>(&'a self, query: &'a str) -> SearchChannels<'a>;
}

impl SearchAPI for Client {
    fn search_categories<'a>(&'a self, query: &'a str) -> SearchCategories<'a> {
        SearchCategories::new(self, query)
    }

    fn search_channels<'a>(&'a self, query: &'a str) -> SearchChannels<'a> {
        SearchChannels::new(self, query)
    }
}
