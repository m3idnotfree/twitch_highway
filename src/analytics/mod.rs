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

endpoints! {
    AnalyticsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
        fn get_extension_analytics(
            &self,
            extension_id: Option<ExtensionId>,
            opts: Option<AnalyticsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionAnalyticsResponse {
            endpoint_type: EndpointType::GetExtensionAnalytics,
            method: Method::GET,
            path: [ANALYTICS, EXTENSIONS],
            query_params: {
                opt(EXTENSION_ID, extension_id),
                opt_into_query(opts),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
        fn get_game_analytics(
            &self,
            game_id: Option<GameId>,
            opts: Option<AnalyticsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> GameAnalyticsResponse {
            endpoint_type: EndpointType::GetGameAnalytics,
            method: Method::GET,
            path: [ANALYTICS, GAMES],
            query_params: {
                opt(GAME_ID, game_id),
                opt_into_query(opts),
                pagination(pagination)
            }
        }
    }
}
