use asknothingx2_util::api::Method;
use request::GameAnalyticsRequest;
use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};

use crate::{
    base::TwitchAPIBase,
    types::{constants::EXTENSIONS, PaginationQuery},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

const ANALYTICS: &str = "analytics";

pub trait AnalyticsAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-extension-analytics
    fn get_extension_analytics(&self) -> TwitchAPIRequest<EmptyBody, ExtensionAnalyticsResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-game-analytics
    fn get_game_analytics(
        &self,
        opts: Option<GameAnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GameAnalyticsResponse>;
}

impl AnalyticsAPI for TwitchAPI {
    fn get_extension_analytics(&self) -> TwitchAPIRequest<EmptyBody, ExtensionAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, EXTENSIONS]);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_game_analytics(
        &self,
        opts: Option<GameAnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GameAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, "games"])
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetGameAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
