pub mod response;
pub mod types;

use response::{CategoriesResponse, ChannelsResponse};

use crate::types::{constants::CHANNELS, PaginationQuery};

endpoints! {
    SearchAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
        fn search_categories(
            &self,
            query: &str,
            pagination: Option<PaginationQuery>,
        ) -> CategoriesResponse {
            endpoint_type: SearchCategories,
            method: GET,
            path: ["search", "categories"],
            query_params: {
                query("query", query),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
        fn search_channels(
            &self,
            query: &str,
            live_only: Option<bool>,
            pagination: Option<PaginationQuery>,
        ) -> ChannelsResponse {
            endpoint_type: SearchChannels,
            method: GET,
            path: ["search", CHANNELS],
            query_params: {
                query("query", query),
                opt("live_only", live_only.map(|x| x.to_string())),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{search::SearchAPI, test_utils::TwitchApiTest};

    api_test!(search_categories, ["fort", None]);
    api_test!(search_channels, ["twitchdev", None, None]);

    #[tokio::test]
    pub(crate) async fn search_channels_2() {
        let suite = TwitchApiTest::new().await;

        suite.search_channels_2().await;

        let _ = suite
            .execute(|api| api.search_channels("a_seagull", Some(true), None))
            .json()
            .await
            .unwrap();
    }
}
