use asknothingx2_util::api::Method;
use request::{
    ExtensionChatMessageIntoRequestBody, RequiredConfiguration, SetConfigurationSegment,
    UpdateExtensoinBitsProductsRequest,
};
use response::{
    ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose, ExtensionSecretsResponse,
    ExtensionsBitsProductsResponse, ExtensionsResponse,
};
use serde_json::Value;
use types::Segment;

use crate::{
    request::{EmptyBody, EndpointType, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BITS, BROADCASTER_ID, CHAT, EXTENSIONS, EXTENSION_ID},
        BroadcasterId, Cost, ExtensionId, JWTToken, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
pub trait ExtensionsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
    fn get_extension_configuration_segment(
        &self,
        jwt_token: JWTToken,
        broadcaster_id: Option<BroadcasterId>,
        extension_id: ExtensionId,
        segment: &[Segment],
    ) -> TwitchAPIRequest<EmptyBody, ConfigurationSegmentResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
    fn set_extension_configuration_segment(
        &self,
        extension_id: ExtensionId,
        segment: Segment,
        opts: Option<SetConfigurationSegment>,
    ) -> TwitchAPIRequest<RequestBody<Value, SetConfigurationSegment>, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: BroadcasterId,
        extension_id: ExtensionId,
        extension_version: &str,
        required_configuration: &str,
    ) -> TwitchAPIRequest<RequiredConfiguration, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
    fn send_extension_pubsub_message(
        &self,
        target: &[&str],
        message: &str,
        broadcaster_id: BroadcasterId,
        is_global_broadcast: Option<bool>,
    ) -> TwitchAPIRequest<RequestBody<Value, EmptyBody>, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
    fn get_extension_live_channels(
        &self,
        extension_id: ExtensionId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionLiveChannelsRespnose>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
    fn get_extension_secrets(
        &self,
        extension_id: ExtensionId,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionSecretsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
    fn create_extension_secret(
        &self,
        extension_id: ExtensionId,
        delay: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionSecretsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
    fn send_extension_chat_message(
        &self,
        broadcaster_id: BroadcasterId,
        text: &str,
        extension_id: ExtensionId,
        extension_version: &str,
    ) -> TwitchAPIRequest<ExtensionChatMessageIntoRequestBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extensions>
    fn get_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
    fn get_released_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
    fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsBitsProductsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
    fn update_extension_bits_products(
        &self,
        sku: &str,
        cost: Cost,
        display_name: &str,
        opts: Option<UpdateExtensoinBitsProductsRequest>,
    ) -> TwitchAPIRequest<
        RequestBody<Value, UpdateExtensoinBitsProductsRequest>,
        ExtensionsBitsProductsResponse,
    >;
}

impl ExtensionsAPI for TwitchAPI {
    fn get_extension_configuration_segment(
        &self,
        jwt_token: JWTToken,
        broadcaster_id: Option<BroadcasterId>,
        extension_id: ExtensionId,
        segments: &[Segment],
    ) -> TwitchAPIRequest<EmptyBody, ConfigurationSegmentResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "configurations"])
            .query_opt(BROADCASTER_ID, broadcaster_id)
            .query(EXTENSION_ID, extension_id)
            .query_extend(segments.iter().map(|segment| ("segment", segment)));

        TwitchAPIRequest::new(
            EndpointType::GetExtensionConfigurationSegment,
            url.build(),
            Method::GET,
            self.build_jwt_headers(&jwt_token).build(),
            EmptyBody,
        )
    }
    fn set_extension_configuration_segment(
        &self,
        extension_id: ExtensionId,
        segment: Segment,
        opts: Option<SetConfigurationSegment>,
    ) -> TwitchAPIRequest<RequestBody<Value, SetConfigurationSegment>, EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "configurations"]);

        let required = serde_json::json!({
            "extension_id": extension_id,
            "segment": segment,
        });

        let request_body = RequestBody::new(required, opts);

        TwitchAPIRequest::new(
            EndpointType::SetExtensionConfigurationSegment,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            request_body,
        )
    }
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: BroadcasterId,
        extension_id: ExtensionId,
        extension_version: &str,
        required_configuration: &str, //request: RequiredConfiguration,
    ) -> TwitchAPIRequest<RequiredConfiguration, EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "required_configuration"])
            .query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::SetExtensionRequiredConfiguration,
            url.build(),
            Method::PUT,
            headers.build(),
            RequiredConfiguration::new(
                extension_id,
                extension_version.to_string(),
                required_configuration.to_string(),
            ),
        )
    }
    fn send_extension_pubsub_message(
        &self,
        target: &[&str],
        message: &str,
        broadcaster_id: BroadcasterId,
        is_global_broadcast: Option<bool>,
    ) -> TwitchAPIRequest<RequestBody<Value, EmptyBody>, EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "pubsub"]);

        let required = if is_global_broadcast.is_some() {
            serde_json::json!({
                "target": target,
                "message": message,
                "broadcaster_id": broadcaster_id,
                "is_global_broadcast": is_global_broadcast.unwrap()
            })
        } else {
            serde_json::json!({
                "target": target,
                "message": message,
                "broadcaster_id": broadcaster_id,
            })
        };

        let request_body = RequestBody::new(required, None::<EmptyBody>);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendExtensionPubSubMessage,
            url.build(),
            Method::POST,
            headers.build(),
            request_body,
        )
    }
    fn get_extension_live_channels(
        &self,
        extension_id: ExtensionId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionLiveChannelsRespnose> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "live"])
            .query(EXTENSION_ID, extension_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionLiveChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_extension_secrets(
        &self,
        extension_id: ExtensionId,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionSecretsResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "jwt", "secrets"])
            .query(EXTENSION_ID, extension_id);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionSecrets,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_extension_secret(
        &self,
        extension_id: ExtensionId,
        delay: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionSecretsResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "jwt", "secrets"])
            .query(EXTENSION_ID, extension_id)
            .query_opt("delay", delay.map(|x| x.to_string()));

        TwitchAPIRequest::new(
            EndpointType::CreateExtensionSecret,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn send_extension_chat_message(
        &self,
        broadcaster_id: BroadcasterId,
        text: &str,
        extension_id: ExtensionId,
        extension_version: &str,
    ) -> TwitchAPIRequest<ExtensionChatMessageIntoRequestBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, CHAT])
            .query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendExtensionChatMessage,
            url.build(),
            Method::POST,
            headers.build(),
            ExtensionChatMessageIntoRequestBody::new(
                text.to_string(),
                extension_id,
                extension_version.to_string(),
            ),
        )
    }
    fn get_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS])
            .query(EXTENSION_ID, extension_id)
            .query_opt("extension_version", extension_version);

        TwitchAPIRequest::new(
            EndpointType::GetExtensions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_released_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "released"])
            .query(EXTENSION_ID, extension_id)
            .query_opt("extension_version", extension_version);

        TwitchAPIRequest::new(
            EndpointType::GetReleasedExtensions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, ExtensionsBitsProductsResponse> {
        let mut url = self.build_url();
        url.path([BITS, EXTENSIONS]).query_opt(
            "should_include_all",
            should_inclue_all.map(|x| x.to_string()),
        );

        TwitchAPIRequest::new(
            EndpointType::GetReleasedExtensions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_extension_bits_products(
        &self,
        sku: &str,
        cost: Cost,
        display_name: &str,
        opts: Option<UpdateExtensoinBitsProductsRequest>,
    ) -> TwitchAPIRequest<
        RequestBody<Value, UpdateExtensoinBitsProductsRequest>,
        ExtensionsBitsProductsResponse,
    > {
        let mut url = self.build_url();
        url.path([BITS, EXTENSIONS]);

        let mut headers = self.build_headers();
        headers.json();
        let required = serde_json::json!({
            "sku": sku,
            "cost": cost,
            "display_name": display_name,
        });

        let request_body = RequestBody::new(required, opts);

        TwitchAPIRequest::new(
            EndpointType::GetReleasedExtensions,
            url.build(),
            Method::PUT,
            headers.build(),
            request_body,
        )
    }
}
