mod builder;
mod response;
mod types;

pub use builder::{
    GetExtensionConfigurationSegment, GetExtensionLiveChannels, SetExtensionConfigurationSegment,
    UpdateExtensionBitsProduct,
};
pub use response::{
    ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose, ExtensionSecretsResponse,
    ExtensionsBitsProductsResponse, ExtensionsResponse,
};
pub use types::{
    BitsProductExtension, Component, ConfigurationLocation, ConfigurationSegment, Extension,
    LiveChannel, Mobile, Panel, Secret, SecretData, Segment, State, SubscriptionsSupportLevel,
    VideoOverlay, Views,
};

use types::{
    ExtensionChatMessageIntoRequestBody, SendExtensionPubSubMessageBody,
    SetExtensionRequiredConfigurationBody,
};

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId, Cost, ExtensionId, JWTToken,
        constants::{
            BITS, BROADCASTER_ID, CHAT, DELAY, EXTENSION_ID, EXTENSION_VERSION, EXTENSIONS, JWT,
            PUBSUB, RELEASED, REQUIRED_CONFIGURATION, SECRETS, SHOULD_INCLUDE_ALL,
        },
    },
};

pub trait ExtensionsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
    fn get_extension_configuration_segment<'a>(
        &'a self,
        jwt_token: JWTToken,
        extension_id: &'a ExtensionId,
        segments: &'a [Segment],
    ) -> GetExtensionConfigurationSegment<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
    fn set_extension_configuration_segment<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
        segment: Segment,
    ) -> SetExtensionConfigurationSegment<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: &BroadcasterId,
        extension_id: &ExtensionId,
        extension_version: &str,
        required_configuration: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
    fn send_extension_pubsub_message(
        &self,
        targets: &[&str],
        message: &str,
        broadcaster_id: &BroadcasterId,
        is_global_broadcast: Option<bool>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
    fn get_extension_live_channels<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionLiveChannels<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
    fn get_extension_secrets(
        &self,
        extension_id: &ExtensionId,
    ) -> impl Future<Output = Result<ExtensionSecretsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
    fn create_extension_secret(
        &self,
        extension_id: &ExtensionId,
        delay: Option<u64>,
    ) -> impl Future<Output = Result<ExtensionSecretsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
    fn send_extension_chat_message(
        &self,
        broadcaster_id: &BroadcasterId,
        text: &str,
        extension_id: &ExtensionId,
        extension_version: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-extensions>
    fn get_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> impl Future<Output = Result<ExtensionsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
    fn get_released_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> impl Future<Output = Result<ExtensionsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
    fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> impl Future<Output = Result<ExtensionsBitsProductsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
    fn update_extension_bits_product<'a>(
        &'a self,
        sku: &'a str,
        cost: Cost,
        display_name: &'a str,
    ) -> UpdateExtensionBitsProduct<'a>;
}

impl ExtensionsAPI for Client {
    fn get_extension_configuration_segment<'a>(
        &'a self,
        jwt_token: JWTToken,
        extension_id: &'a ExtensionId,
        segments: &'a [Segment],
    ) -> GetExtensionConfigurationSegment<'a> {
        GetExtensionConfigurationSegment::new(self, jwt_token, extension_id, segments)
    }

    fn set_extension_configuration_segment<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
        segment: Segment,
    ) -> SetExtensionConfigurationSegment<'a> {
        SetExtensionConfigurationSegment::new(self, extension_id, segment)
    }

    async fn set_extension_required_configuration(
        &self,
        broadcaster_id: &BroadcasterId,
        extension_id: &ExtensionId,
        extension_version: &str,
        required_configuration: &str,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, REQUIRED_CONFIGURATION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        let req = self
            .http_client()
            .put(url)
            .json(&SetExtensionRequiredConfigurationBody {
                extension_id,
                extension_version,
                required_configuration,
            });
        self.no_content(req).await
    }

    async fn send_extension_pubsub_message(
        &self,
        targets: &[&str],
        message: &str,
        broadcaster_id: &BroadcasterId,
        is_global_broadcast: Option<bool>,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, PUBSUB]);

        let req = self
            .http_client()
            .post(url)
            .json(&SendExtensionPubSubMessageBody {
                target: targets,
                message,
                broadcaster_id,
                is_global_broadcast,
            });
        self.no_content(req).await
    }

    fn get_extension_live_channels<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionLiveChannels<'a> {
        GetExtensionLiveChannels::new(self, extension_id)
    }

    async fn get_extension_secrets(
        &self,
        extension_id: &ExtensionId,
    ) -> Result<ExtensionSecretsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, JWT, SECRETS]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, extension_id);

        self.json(self.http_client().get(url)).await
    }

    async fn create_extension_secret(
        &self,
        extension_id: &ExtensionId,
        delay: Option<u64>,
    ) -> Result<ExtensionSecretsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, JWT, SECRETS]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, extension_id);
        if let Some(d) = delay {
            url.query_pairs_mut().append_pair(DELAY, &d.to_string());
        }

        self.json(self.http_client().post(url)).await
    }

    async fn send_extension_chat_message(
        &self,
        broadcaster_id: &BroadcasterId,
        text: &str,
        extension_id: &ExtensionId,
        extension_version: &str,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([EXTENSIONS, CHAT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        let req = self
            .http_client()
            .post(url)
            .json(&ExtensionChatMessageIntoRequestBody {
                text,
                extension_id,
                extension_version,
            });
        self.no_content(req).await
    }

    async fn get_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> Result<ExtensionsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(EXTENSIONS);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, extension_id);
        if let Some(v) = extension_version {
            url.query_pairs_mut().append_pair(EXTENSION_VERSION, v);
        }

        self.json(self.http_client().get(url)).await
    }

    async fn get_released_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> Result<ExtensionsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, RELEASED]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, extension_id);
        if let Some(v) = extension_version {
            url.query_pairs_mut().append_pair(EXTENSION_VERSION, v);
        }

        self.json(self.http_client().get(url)).await
    }

    async fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> Result<ExtensionsBitsProductsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([BITS, EXTENSIONS]);

        if let Some(all) = should_inclue_all {
            url.query_pairs_mut()
                .append_pair(SHOULD_INCLUDE_ALL, &all.to_string());
        }

        self.json(self.http_client().get(url)).await
    }

    fn update_extension_bits_product<'a>(
        &'a self,
        sku: &'a str,
        cost: Cost,
        display_name: &'a str,
    ) -> UpdateExtensionBitsProduct<'a> {
        UpdateExtensionBitsProduct::new(self, sku, cost, display_name)
    }
}
