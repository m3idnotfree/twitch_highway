mod builder;
mod response;
mod types;

pub use builder::{SearchCategories, SearchChannels};
pub use response::{CategoriesResponse, ChannelsResponse};
pub use types::Channel;

use crate::Client;

pub trait SearchAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories<'a>(&'a self, query: &'a str) -> SearchCategories<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#search-channels>
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
