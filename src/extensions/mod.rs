use asknothingx2_util::api::Method;
use request::{
    ExtensionChatMessageIntoRequestBody, RequiredConfiguration, SetConfigurationSegment,
    UpdateExtensoinBitsProductsRequest,
};
use response::{
    ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose, ExtensionSecretsResponse,
    ExtensionsBitsProductsResponse, ExtensionsResponse,
};
use types::Segment;

use crate::{
    request::{EndpointType, NoContent, RequestBody, TwitchAPIRequest},
    types::{BroadcasterId, Cost, ExtensionId, JWTToken, PaginationQuery},
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    trait ExtensionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
        fn get_extension_configuration_segment(
            &self,
            jwt_token: JWTToken,
            broadcaster_id: Option<BroadcasterId>,
            extension_id: ExtensionId,
            segment: &[Segment],
        ) -> ConfigurationSegmentResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
        fn set_extension_configuration_segment(
            &self,
            extension_id: ExtensionId,
            segment: Segment,
            opts: Option<SetConfigurationSegment>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
        fn set_extension_required_configuration(
            &self,
            broadcaster_id: BroadcasterId,
            extension_id: ExtensionId,
            extension_version: &str,
            required_configuration: &str,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
        fn send_extension_pubsub_message(
            &self,
            target: &[&str],
            message: &str,
            broadcaster_id: BroadcasterId,
            is_global_broadcast: Option<bool>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
        fn get_extension_live_channels(
            &self,
            extension_id: ExtensionId,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionLiveChannelsRespnose;
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
        fn get_extension_secrets(
            &self,
            extension_id: ExtensionId,
        ) -> ExtensionSecretsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
        fn create_extension_secret(
            &self,
            extension_id: ExtensionId,
            delay: Option<u64>,
        ) -> ExtensionSecretsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
        fn send_extension_chat_message(
            &self,
            broadcaster_id: BroadcasterId,
            text: &str,
            extension_id: ExtensionId,
            extension_version: &str,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-extensions>
        fn get_extensions(
            &self,
            extension_id: ExtensionId,
            extension_version: Option<&str>,
        ) -> ExtensionsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
        fn get_released_extensions(
            &self,
            extension_id: ExtensionId,
            extension_version: Option<&str>,
        ) -> ExtensionsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
        fn get_extension_bits_products(
            &self,
            should_inclue_all: Option<bool>,
        ) -> ExtensionsBitsProductsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
        fn update_extension_bits_products(
            &self,
            sku: &str,
            cost: Cost,
            display_name: &str,
            opts: Option<UpdateExtensoinBitsProductsRequest>,
        ) -> ExtensionsBitsProductsResponse;
    }
    impl {
        get_extension_configuration_segment => {
            endpoint_type: EndpointType::GetExtensionConfigurationSegment,
            method: Method::GET,
            path: ["extensions", "configurations"],
            query_params: {
                opt("broadcaster_id", broadcaster_id),
                query("extension_id", extension_id),
                extend(segment.iter().map(|s| ("segment", s)))
            },
            headers: [jwt, jwt_token]
        }
        set_extension_configuration_segment => {
            endpoint_type: EndpointType::SetExtensionConfigurationSegment,
            method: Method::PUT,
            path: ["extensions", "configurations"],
            body: {
                let required = serde_json::json!({
                    "extension_id": extension_id,
                    "segment": segment,
                });
                RequestBody::new(required, opts).into_json()
            }
        }
        set_extension_required_configuration => {
            endpoint_type: EndpointType::SetExtensionRequiredConfiguration,
            method: Method::PUT,
            path: ["extensions", "required_configuration"],
            query_params: {
                query("broadcaster_id", broadcaster_id)
            },
            headers: [json],
            body: RequiredConfiguration::new(extension_id, extension_version, required_configuration).into_json()
        }
        send_extension_pubsub_message => {
            endpoint_type: EndpointType::SendExtensionPubSubMessage,
            method: Method::POST,
            path: ["extensions", "pubsub"],
            headers: [json],
            body: {
                let required = if let Some(is_global) = is_global_broadcast {
                    serde_json::json!({
                        "target": target,
                        "message": message,
                        "broadcaster_id": broadcaster_id,
                        "is_global_broadcast": is_global
                    })
                } else {
                    serde_json::json!({
                        "target": target,
                        "message": message,
                        "broadcaster_id": broadcaster_id,
                    })
                };
                RequestBody::new(required, None::<NoContent>).into_json()
            }
        }
        get_extension_live_channels => {
            endpoint_type: EndpointType::GetExtensionLiveChannels,
            method: Method::GET,
            path: ["extensions", "live"],
            query_params: {
                query("extension_id", extension_id),
                opt_into_query(pagination)
            }
        }
        get_extension_secrets => {
            endpoint_type: EndpointType::GetExtensionSecrets,
            method: Method::GET,
            path: ["extensions", "jwt", "secrets"],
            query_params: {
                query("extension_id", extension_id)
            }
        }
        create_extension_secret => {
            endpoint_type: EndpointType::CreateExtensionSecret,
            method: Method::POST,
            path: ["extensions", "jwt", "secrets"],
            query_params: {
                query("extension_id", extension_id),
                opt("delay", delay.as_ref().map(|d| d.to_string()))
            }
        }
        send_extension_chat_message => {
            endpoint_type: EndpointType::SendExtensionChatMessage,
            method: Method::POST,
            path: ["extensions", "chat"],
            query_params: {
                query("broadcaster_id", broadcaster_id)
            },
            headers: [json],
            body: ExtensionChatMessageIntoRequestBody::new(text, extension_id, extension_version).into_json()
        }
        get_extensions => {
            endpoint_type: EndpointType::GetExtensions,
            method: Method::GET,
            path: ["extensions"],
            query_params: {
                query("extension_id", extension_id),
                opt("extension_version", extension_version)
            }
        }
        get_released_extensions => {
            endpoint_type: EndpointType::GetReleasedExtensions,
            method: Method::GET,
            path: ["extensions", "released"],
            query_params: {
                query("extension_id", extension_id),
                opt("extension_version", extension_version)
            }
        }
        get_extension_bits_products => {
            endpoint_type: EndpointType::GetExtensionBitsProducts,
            method: Method::GET,
            path: ["bits", "extensions"],
            query_params: {
                opt("should_include_all", should_inclue_all.as_ref().map(|b| b.to_string()))
            }
        }
        update_extension_bits_products => {
            endpoint_type: EndpointType::GetReleasedExtensions,
            method: Method::PUT,
            path: ["bits", "extensions"],
            headers: [json],
            body: {
                let required = serde_json::json!({
                    "sku": sku,
                    "cost": cost,
                    "display_name": display_name,
                });
                RequestBody::new(required, opts).into_json()
            }
        }
    }
}

#[cfg(test)]
mod extensions_api_tests {
    use crate::{
        extensions::{types::Segment, ExtensionsAPI},
        test_utils::TwitchApiTest,
        types::{BroadcasterId, ExtensionId, JWTToken, PaginationQuery},
    };

    #[tokio::test]
    async fn get_extension_configuration_segment_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_success().await;

        let jwt_token = JWTToken::new("test_jwt_token");
        let segments = vec![Segment::Broadcaster];

        let response = suite
            .execute("/extensions/configurations", |api| {
                api.get_extension_configuration_segment(
                    jwt_token,
                    Some(BroadcasterId::new("123456789")),
                    ExtensionId::new("ext123"),
                    &segments,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let segment = &response.data[0];
        assert_eq!(segment.segment.as_ref(), "broadcaster");
        assert_eq!(
            segment.broadcaster_id.as_ref().unwrap().as_str(),
            "123456789"
        );
        assert_eq!(segment.content, "test_config_content");
        assert_eq!(segment.version, "1.0.0");
    }

    #[tokio::test]
    async fn get_extension_live_channels_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/extensions/live", |api| {
                api.get_extension_live_channels(ExtensionId::new("ext456"), Some(pagination))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());

        let first_channel = &response.data[0];
        assert_eq!(first_channel.broadcaster_name, "LiveStreamer1");
        assert_eq!(first_channel.game_name, "Just Chatting");

        let second_channel = &response.data[1];
        assert_eq!(second_channel.broadcaster_name, "LiveStreamer2");
        assert_eq!(second_channel.game_name, "VALORANT");
    }

    #[tokio::test]
    async fn get_extension_secrets_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_success().await;

        let response = suite
            .execute("/extensions/jwt/secrets", |api| {
                api.get_extension_secrets(ExtensionId::new("ext789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let secret_data = &response.data[0];
        assert_eq!(secret_data.format_version, 1);
        assert_eq!(secret_data.secrets.len(), 1);
        assert_eq!(secret_data.secrets[0].content, "active_secret_12345");
    }

    #[tokio::test]
    async fn create_extension_secret_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_success().await;

        let response = suite
            .execute("/extensions/jwt/secrets", |api| {
                api.create_extension_secret(ExtensionId::new("ext999"), Some(300))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let secret_data = &response.data[0];
        assert_eq!(secret_data.secrets.len(), 2);
        assert_eq!(secret_data.secrets[0].content, "old_secret_content");
        assert_eq!(secret_data.secrets[1].content, "new_secret_content");
    }

    #[tokio::test]
    async fn get_extension_bits_products_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_success().await;

        let response = suite
            .execute("/bits/extensions", |api| {
                api.get_extension_bits_products(Some(true))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let product = &response.data[0];
        assert_eq!(product.sku, "power_up_001");
        assert_eq!(product.cost.amount, 150);
        assert_eq!(product.display_name, "Power Up Pack");
        assert!(!product.in_development);
        assert!(product.is_broadcast);
    }

    #[tokio::test]
    async fn extensions_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_extensions_failure().await;

        let jwt_token = JWTToken::new("invalid_token");
        let segments = vec![Segment::Global];

        let response = suite
            .execute("/extensions/configurations", |api| {
                api.get_extension_configuration_segment(
                    jwt_token,
                    None,
                    ExtensionId::new("ext123"),
                    &segments,
                )
            })
            .send()
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
