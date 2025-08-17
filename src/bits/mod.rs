use asknothingx2_util::api::Method;
use request::BitsLeaderboardRequest;
use response::{BitsLeaderboardResponse, CheermotesResponse, ExtensionTransactionsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BITS, BROADCASTER_ID, EXTENSIONS, EXTENSION_ID, ID},
        BroadcasterId, ExtensionId, Id, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    BitsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
        fn get_bits_leaderboard(
            &self,
            opts: Option<BitsLeaderboardRequest>,
        ) -> BitsLeaderboardResponse {
            endpoint_type: EndpointType::GetBitsLeaderboard,
            method: Method::GET,
            path: [BITS, "leaderboard"],
            query_params: {
                opt_into_query(opts)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
        fn get_cheermotes(
            &self,
            broadcaster_id: Option<BroadcasterId>,
        ) -> CheermotesResponse {
            endpoint_type: EndpointType::GetCheermotes,
            method: Method::GET,
            path: [BITS, "cheermotes"],
            query_params: {
                opt(BROADCASTER_ID, broadcaster_id)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
        fn get_extension_transactions(
            &self,
            extension_id: ExtensionId,
            id: Option<Id>,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionTransactionsResponse {
            endpoint_type: EndpointType::GetExtensionTransactions,
            method: Method::GET,
            path: [EXTENSIONS, "transactions"],
            query_params: {
                query(EXTENSION_ID, extension_id),
                opt(ID, id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bits::{request::BitsLeaderboardRequest, BitsAPI},
        test_utils::TwitchApiTest,
        types::{BroadcasterId, ExtensionId, Id, PaginationQuery},
    };

    #[tokio::test]
    pub(crate) async fn get_bits_leaderboard() {
        let suite = TwitchApiTest::new().await;

        suite.mock_bits_success().await;

        let leaderboard_request = BitsLeaderboardRequest::new().count(10).period("week");

        let response = suite
            .execute("/bits/leaderboard", |api| {
                api.get_bits_leaderboard(Some(leaderboard_request))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].user_id.as_str(), "158010205");
        assert_eq!(response.total, 2);
    }

    #[tokio::test]
    pub(crate) async fn get_cheermotes() {
        let suite = TwitchApiTest::new().await;

        suite.mock_bits_success().await;

        let response = suite
            .execute("/bits/cheermotes", |api| {
                api.get_cheermotes(Some(BroadcasterId::new("broadcaster123")))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].prefix, "Cheer");
    }

    #[tokio::test]
    pub(crate) async fn get_extension_transactions() {
        let suite = TwitchApiTest::new().await;

        suite.mock_bits_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/extensions/transactions", |api| {
                api.get_extension_transactions(
                    ExtensionId::new("ext123456"),
                    Some(Id::new("trans123456")),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id.as_str(), "trans123456");
        assert!(response.pagination.is_some());
    }

    #[tokio::test]
    async fn get_bits_leaderboard_no_options() {
        let suite = TwitchApiTest::new().await;

        suite.mock_bits_extra().await;

        let response = suite
            .execute("/bits/leaderboard", |api| api.get_bits_leaderboard(None))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 0);
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn bits_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_bits_failure().await;
        let response = suite
            .execute("/bits/leaderboard", |api| api.get_bits_leaderboard(None))
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
