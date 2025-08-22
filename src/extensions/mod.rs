use asknothingx2_util::api::Method;
use request::{
    ExtensionChatMessageIntoRequestBody, RequiredConfiguration, UpdateExtensoinBitsProductsRequest,
};
use response::{
    ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose, ExtensionSecretsResponse,
    ExtensionsBitsProductsResponse, ExtensionsResponse,
};
use serde_json::json;
use types::Segment;

use crate::{
    extensions::request::RequestConfigurationSegment,
    request::{EndpointType, NoContent, RequestBody, TwitchAPIRequest},
    types::{BroadcasterId, Cost, ExtensionId, JWTToken, PaginationQuery},
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    ExtensionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
        fn get_extension_configuration_segment(
            &self,
            jwt_token: JWTToken,
            extension_id: &ExtensionId,
            segment: &[Segment],
            broadcaster_id: Option<&BroadcasterId>,
        ) -> ConfigurationSegmentResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
        fn set_extension_configuration_segment(
            &self,
            extension_id: &ExtensionId,
            segment: Segment,
            opts: Option<RequestConfigurationSegment>,
        // ) -> ConfigurationSegmentResponse {
        ) -> NoContent {
            endpoint_type: EndpointType::SetExtensionConfigurationSegment,
            method: Method::PUT,
            path: ["extensions", "configurations"],
            headers: [json],
            body:  {
            let req = json!({
                "extension_id": extension_id,
                "segment": segment,
            });

            RequestBody::new(req, opts).into_json()
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
        fn set_extension_required_configuration(
            &self,
            broadcaster_id: &BroadcasterId,
            extension_id: &ExtensionId,
            extension_version: &str,
            required_configuration: &str,
        ) -> NoContent {
            endpoint_type: EndpointType::SetExtensionRequiredConfiguration,
            method: Method::PUT,
            path: ["extensions", "required_configuration"],
            query_params: {
                query("broadcaster_id", broadcaster_id)
            },
            headers: [json],
            body: RequiredConfiguration::new(extension_id, extension_version, required_configuration).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
        fn send_extension_pubsub_message(
            &self,
            target: &[&str],
            message: &str,
            broadcaster_id: &BroadcasterId,
            is_global_broadcast: Option<bool>,
        ) -> NoContent {
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

        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
        fn get_extension_live_channels(
            &self,
            extension_id: &ExtensionId,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionLiveChannelsRespnose {
            endpoint_type: EndpointType::GetExtensionLiveChannels,
            method: Method::GET,
            path: ["extensions", "live"],
            query_params: {
                query("extension_id", extension_id),
                opt_into_query(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
        fn get_extension_secrets(
            &self,
            extension_id: &ExtensionId,
        ) -> ExtensionSecretsResponse {
            endpoint_type: EndpointType::GetExtensionSecrets,
            method: Method::GET,
            path: ["extensions", "jwt", "secrets"],
            query_params: {
                query("extension_id", extension_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
        fn create_extension_secret(
            &self,
            extension_id: &ExtensionId,
            delay: Option<u64>,
        ) -> ExtensionSecretsResponse {
            endpoint_type: EndpointType::CreateExtensionSecret,
            method: Method::POST,
            path: ["extensions", "jwt", "secrets"],
            query_params: {
                query("extension_id", extension_id),
                opt("delay", delay.as_ref().map(|d| d.to_string()))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
        fn send_extension_chat_message(
            &self,
            broadcaster_id: &BroadcasterId,
            text: &str,
            extension_id: &ExtensionId,
            extension_version: &str,
        ) -> NoContent {
            endpoint_type: EndpointType::SendExtensionChatMessage,
            method: Method::POST,
            path: ["extensions", "chat"],
            query_params: {
                query("broadcaster_id", broadcaster_id)
            },
            headers: [json],
            body: ExtensionChatMessageIntoRequestBody::new(text, extension_id, extension_version).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-extensions>
        fn get_extensions(
            &self,
            extension_id: &ExtensionId,
            extension_version: Option<&str>,
        ) -> ExtensionsResponse {
            endpoint_type: EndpointType::GetExtensions,
            method: Method::GET,
            path: ["extensions"],
            query_params: {
                query("extension_id", extension_id),
                opt("extension_version", extension_version)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
        fn get_released_extensions(
            &self,
            extension_id: &ExtensionId,
            extension_version: Option<&str>,
        ) -> ExtensionsResponse {
            endpoint_type: EndpointType::GetReleasedExtensions,
            method: Method::GET,
            path: ["extensions", "released"],
            query_params: {
                query("extension_id", extension_id),
                opt("extension_version", extension_version)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
        fn get_extension_bits_products(
            &self,
            should_inclue_all: Option<bool>,
        ) -> ExtensionsBitsProductsResponse {
            endpoint_type: EndpointType::GetExtensionBitsProducts,
            method: Method::GET,
            path: ["bits", "extensions"],
            query_params: {
                opt("should_include_all", should_inclue_all.as_ref().map(|b| b.to_string()))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
        fn update_extension_bits_product(
            &self,
            sku: &str,
            cost: Cost,
            display_name: &str,
            opts: Option<UpdateExtensoinBitsProductsRequest>,
        ) -> ExtensionsBitsProductsResponse {
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
mod tests {
    use crate::{
        extensions::{
            request::{RequestConfigurationSegment, UpdateExtensoinBitsProductsRequest},
            types::Segment,
            ExtensionsAPI,
        },
        types::{BroadcasterId, Cost, CostType, ExtensionId, JWTToken},
    };

    api_test!(
        get_extension_configuration_segment,
        [
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
            None,
        ]
    );
    api_test!(
        set_extension_configuration_segment,
        [
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Segment::Global,
            Some(
                RequestConfigurationSegment::new()
                    .version("0.0.1")
                    .content("hello config!")
            )
        ]
    );
    api_test!(
        set_extension_required_configuration,
        [
            &BroadcasterId::from("274637212"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            "0.0.1",
            "RCS",
        ]
    );
    api_test!(
        send_extension_pubsub_message,
        [
            &["broadcast"],
            "hello world!",
            &BroadcasterId::from("141981764"),
            None
        ]
    );
    api_test!(
        get_extension_live_channels,
        [&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"), None]
    );
    api_test!(
        get_extension_secrets,
        [&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2")]
    );
    api_test!(
        create_extension_secret,
        [
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some(600)
        ]
    );
    api_test!(
        send_extension_chat_message,
        [
            &BroadcasterId::from("237757755"),
            "Hello",
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            "0.0.9",
        ]
    );
    api_test!(
        get_extensions,
        [
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some("0.0.9")
        ]
    );
    api_test!(
        get_released_extensions,
        [
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Some("0.0.9")
        ]
    );
    api_test!(get_extension_bits_products, [Some(true)]);
    api_test!(
        update_extension_bits_product,
        [
            "1010",
            Cost::new(990, CostType::Bits),
            "Rusty Crate 2",
            Some(
                UpdateExtensoinBitsProductsRequest::new()
                    .in_development(true)
                    .is_broadcast(true)
                    .expiration("2021-05-18T09:10:13.397Z")
            ),
        ]
    );

    api_test!(extra
        get_extension_configuration_segment,
        get_extension_configuration_segment2,
        [
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
            None,
        ]
    );
}
