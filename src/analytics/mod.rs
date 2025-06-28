use asknothingx2_util::api::Method;
use request::AnalyticsRequest;
use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{ANALYTICS, EXTENSIONS, EXTENSION_ID, GAMES, GAME_ID},
        ExtensionId, GameId, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "analytics")))]
pub trait AnalyticsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
    fn get_extension_analytics(
        &self,
        extension_id: Option<ExtensionId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ExtensionAnalyticsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
    fn get_game_analytics(
        &self,
        game_id: Option<GameId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<GameAnalyticsResponse>;
}

impl AnalyticsAPI for TwitchAPI {
    fn get_extension_analytics(
        &self,
        extension_id: Option<ExtensionId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ExtensionAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, EXTENSIONS])
            .query_opt(EXTENSION_ID, extension_id);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetExtensionAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_game_analytics(
        &self,
        game_id: Option<GameId>,
        opts: Option<AnalyticsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<GameAnalyticsResponse> {
        let mut url = self.build_url();
        url.path([ANALYTICS, GAMES]).query_opt(GAME_ID, game_id);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetGameAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
