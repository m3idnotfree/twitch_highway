use asknothingx2_util::api::Method;
use request::{
    ExtensionChatMessageRequestBody, PubSubMessageRequest, RequiredConfiguration,
    SetConfigurationSegment,
};
use types::{BitsProductExtension, Segment};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, ExtensionId, JWTToken, AFTER, BROADCASTER_ID, EXTENSIONS, FIRST},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

const EXTENSION_ID: &str = "extension_id";

pub trait ExtensionsAPI: TwitchAPIBase {
    fn get_extension_configuration_segment(
        &self,
        jwt_token: JWTToken,
        broadcaster_id: Option<BroadcasterId>,
        extension_id: ExtensionId,
        segment: &[Segment],
    ) -> TwitchAPIRequest<EmptyBody>;
    fn set_extension_configuration_segment(
        &self,
        configuration: SetConfigurationSegment,
    ) -> TwitchAPIRequest<SetConfigurationSegment>;
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: BroadcasterId,
        request: RequiredConfiguration,
    ) -> TwitchAPIRequest<RequiredConfiguration>;
    fn send_extension_pubsub_message(
        &self,
        request: PubSubMessageRequest,
    ) -> TwitchAPIRequest<PubSubMessageRequest>;
    fn get_extension_livemchannels(
        &self,
        extension_id: ExtensionId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_extension_secrets(&self, extension_id: ExtensionId) -> TwitchAPIRequest<EmptyBody>;
    fn create_extension_secret(
        &self,
        extension_id: ExtensionId,
        delay: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn send_extension_chat_message(
        &self,
        broadcaster_id: BroadcasterId,
        request: ExtensionChatMessageRequestBody,
    ) -> TwitchAPIRequest<ExtensionChatMessageRequestBody>;
    fn get_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_released_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_extension_bits_products(
        &self,
        request: BitsProductExtension,
    ) -> TwitchAPIRequest<BitsProductExtension>;
}

impl ExtensionsAPI for TwitchAPI {
    fn get_extension_configuration_segment(
        &self,
        jwt_token: JWTToken,
        broadcaster_id: Option<BroadcasterId>,
        extension_id: ExtensionId,
        segments: &[Segment],
    ) -> TwitchAPIRequest<EmptyBody> {
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
        configuration: SetConfigurationSegment,
    ) -> TwitchAPIRequest<SetConfigurationSegment> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "configurations"]);

        TwitchAPIRequest::new(
            EndpointType::SetExtensionConfigurationSegment,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            configuration,
        )
    }
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: BroadcasterId,
        request: RequiredConfiguration,
    ) -> TwitchAPIRequest<RequiredConfiguration> {
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
            request,
        )
    }
    fn send_extension_pubsub_message(
        &self,
        request: PubSubMessageRequest,
    ) -> TwitchAPIRequest<PubSubMessageRequest> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "pubsub"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendExtensionPubSubMessage,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn get_extension_livemchannels(
        &self,
        extension_id: ExtensionId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "live"])
            .query(EXTENSION_ID, extension_id)
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionLiveChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_extension_secrets(&self, extension_id: ExtensionId) -> TwitchAPIRequest<EmptyBody> {
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
    ) -> TwitchAPIRequest<EmptyBody> {
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
        request: ExtensionChatMessageRequestBody,
    ) -> TwitchAPIRequest<ExtensionChatMessageRequestBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "chat"])
            .query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendExtensionChatMessage,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn get_extensions(
        &self,
        extension_id: ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
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
    ) -> TwitchAPIRequest<EmptyBody> {
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
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["bits", EXTENSIONS]).query_opt(
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
        request: BitsProductExtension,
    ) -> TwitchAPIRequest<BitsProductExtension> {
        let mut url = self.build_url();
        url.path(["bits", EXTENSIONS]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::GetReleasedExtensions,
            url.build(),
            Method::PUT,
            headers.build(),
            request,
        )
    }
}
