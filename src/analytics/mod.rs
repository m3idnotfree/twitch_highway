mod builder;
mod response;
mod types;

pub use builder::{GetExtensionAnalytics, GetGameAnalytics};
pub use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};
pub use types::{AnalyticsType, ExtensionAnalytic, GameAnalytic};

use crate::Client;

pub trait AnalyticsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn get_extension_analytics<'a>(&'a self) -> GetExtensionAnalytics<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn get_game_analytics<'a>(&'a self) -> GetGameAnalytics<'a>;
}

impl AnalyticsAPI for Client {
    fn get_extension_analytics<'a>(&'a self) -> GetExtensionAnalytics<'a> {
        GetExtensionAnalytics::new(self)
    }
    fn get_game_analytics<'a>(&'a self) -> GetGameAnalytics<'a> {
        GetGameAnalytics::new(self)
    }
}
