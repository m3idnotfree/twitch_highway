use asknothingx2_util::api::Method;
use request::AnalyticsRequest;
use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};

use crate::{
    base::TwitchAPIBase,
    types::{
        constants::{ANALYTICS, EXTENSIONS, EXTENSION_ID, GAMES, GAME_ID},
        ExtensionId, GameId, PaginationQuery,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "analytics")))]
pub trait AnalyticsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn get_extension_analytics(
        &self,
        extension_id: Option<ExtensionId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionAnalyticsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn get_game_analytics(
        &self,
        game_id: Option<GameId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GameAnalyticsResponse>;
}

impl AnalyticsAPI for TwitchAPI {
    fn get_extension_analytics(
        &self,
        extension_id: Option<ExtensionId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, EXTENSIONS])
            .query_opt(EXTENSION_ID, extension_id)
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

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
        game_id: Option<GameId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GameAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, GAMES])
            .query_opt(GAME_ID, game_id)
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
