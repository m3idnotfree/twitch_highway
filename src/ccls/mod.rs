use asknothingx2_util::api::Method;
use request::CclsLocale;
use response::CclsResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "ccls")))]
    trait CclsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-content-classification-labels>
        fn get_content_classification_labels(
            &self,
            locale: Option<CclsLocale>,
        ) -> CclsResponse;
    }
    impl {
        get_content_classification_labels => {
            endpoint_type: EndpointType::GetContentClassificationLabels,
            method: Method::GET,
            path: ["content_classification_labels"],
            query_params: {
                opt("locale", locale)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ccls::{request::CclsLocale, CclsAPI},
        test_utils::TwitchApiTest,
    };

    #[tokio::test]
    async fn get_content_classification_labels_with_locale() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ccls_success().await;

        let response = suite
            .execute("/content_classification_labels", |api| {
                api.get_content_classification_labels(Some(CclsLocale::enUS))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id.as_str(), "DrugsIntoxication");
        assert_eq!(response.data[1].id.as_str(), "SexualThemes");
    }

    #[tokio::test]
    async fn get_content_classification_labels_without_locale() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ccls_success_without_locale().await;

        let response = suite
            .execute("/content_classification_labels", |api| {
                api.get_content_classification_labels(None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id.as_str(), "ViolentGraphic");
        assert_eq!(response.data[1].id.as_str(), "Gambling");
    }

    #[tokio::test]
    async fn ccls_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ccls_failure().await;

        let response = suite
            .execute("/content_classification_labels", |api| {
                api.get_content_classification_labels(Some(CclsLocale::enUS))
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
