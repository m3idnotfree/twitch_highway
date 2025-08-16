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

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "analytics")))]
    trait AnalyticsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-analytics>
        fn get_extension_analytics(
            &self,
            extension_id: Option<ExtensionId>,
            opts: Option<AnalyticsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionAnalyticsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-game-analytics>
        fn get_game_analytics(
            &self,
            game_id: Option<GameId>,
            opts: Option<AnalyticsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> GameAnalyticsResponse;
    }
    impl {
        get_extension_analytics => {
            endpoint_type: EndpointType::GetExtensionAnalytics,
            method: Method::GET,
            path: [ANALYTICS, EXTENSIONS],
            query_params: {
                opt(EXTENSION_ID, extension_id),
                opt_into_query(opts),
                pagination(pagination)
            }
        }
        get_game_analytics => {
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
    use chrono::{DateTime, FixedOffset};
    use std::str::FromStr;

    use crate::{
        analytics::{
            request::{AnalyticsRequest, AnalyticsType},
            AnalyticsAPI,
        },
        test_utils::TwitchApiTest,
        types::{ExtensionId, GameId, PaginationQuery},
    };

    #[tokio::test]
    pub(crate) async fn get_extension_analytics() {
        let suite = TwitchApiTest::new().await;

        suite.mock_analytics_success().await;

        let analytics_request = AnalyticsRequest::new().kind(AnalyticsType::OverviewV2);
        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/analytics/extensions", |api| {
                api.get_extension_analytics(
                    Some(ExtensionId::new("ext123456")),
                    Some(analytics_request),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].extension_id.as_str(), "ext123456");
        assert!(response.pagination.is_some());
    }

    #[tokio::test]
    pub(crate) async fn get_game_analytics() {
        let started_at = DateTime::<FixedOffset>::from_str("2023-12-01T00:00:00Z").unwrap();
        let ended_at = DateTime::<FixedOffset>::from_str("2023-12-01T23:59:59Z").unwrap();

        let analytics_request = AnalyticsRequest::new()
            .started_at(started_at)
            .ended_at(ended_at);

        let suite = TwitchApiTest::new().await;

        suite.mock_analytics_success().await;

        let response = suite
            .execute("/analytics/games", |api| {
                api.get_game_analytics(Some(GameId::new("12345")), Some(analytics_request), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].game_id.as_str(), "12345");
        assert!(response.pagination.is_none());
    }

    #[tokio::test]
    async fn get_extension_analytics_no_optional_params() {
        let suite = TwitchApiTest::new().await;

        suite.mock_analytics_extra().await;

        let response = suite
            .execute("/analytics/extensions", |api| {
                api.get_extension_analytics(None, None, None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
    }

    #[tokio::test]
    async fn analytics_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_analytics_failure().await;

        let response = suite
            .execute("/analytics/extensions", |api| {
                api.get_game_analytics(None, None, None)
            })
            .json()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
