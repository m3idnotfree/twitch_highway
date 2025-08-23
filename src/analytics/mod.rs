pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::AnalyticsRequest;
use response::{ExtensionAnalyticsResponse, GameAnalyticsResponse};

use crate::{
    request::EndpointType,
    types::{
        constants::{EXTENSIONS, EXTENSION_ID, GAMES, GAME_ID},
        ExtensionId, GameId, PaginationQuery,
    },
};

const ANALYTICS: &str = "analytics";

endpoints! {
    AnalyticsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
        fn get_extension_analytics(
            &self,
            extension_id: Option<&ExtensionId>,
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
            game_id: Option<&GameId>,
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

#[cfg(test)]
mod tests {
    use crate::{
        analytics::{request::AnalyticsRequest, AnalyticsAPI},
        types::{GameId, PaginationQuery},
    };

    api_test!(get_extension_analytics, [None, None, None]);
    api_test!(
        get_game_analytics,
        [
            Some(&GameId::from("493057")),
            Some(
                AnalyticsRequest::new()
                    .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
                    .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
            ),
            None
        ]
    );

    api_test!(extra
        get_game_analytics,
        get_game_analytics2,
        [
            None,
            None,
            Some(PaginationQuery::new().first(5))
        ]
    );
}
