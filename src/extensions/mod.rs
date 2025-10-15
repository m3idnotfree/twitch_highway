mod builder;
mod response;
mod types;

pub use builder::{
    GetExtensionConfigurationSegmentBuilder, GetExtensionLiveChannelsBuilder,
    SetExtensionConfigurationSegmentBuilder, UpdateExtensionBitsProductBuilder,
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

use types::{SendExtensionPubSubMessageBody, SetExtensionRequiredConfigurationBody};

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{
            BITS, BROADCASTER_ID, CHAT, DELAY, EXTENSIONS, EXTENSION_ID, EXTENSION_VERSION, JWT,
            PUBSUB, RELEASED, REQUIRED_CONFIGURATION, SECRETS, SHOULD_INCLUDE_ALL,
        },
        BroadcasterId, Cost, ExtensionId, JWTToken,
    },
    TwitchAPI,
};

pub trait ExtensionsAPI {
    /// Gets the specified configuration segment from the specified extension
    ///
    /// # Arguments
    ///
    /// * `jwt_token` -
    /// * `extension_id` -
    /// * `segments` -
    /// * `broadcaster_id` -
    /// * `` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetExtensionConfigurationSegmentBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::{ExtensionsAPI, Segment},
    ///     types::{ExtensionId, JWTToken},
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_extension_configuration_segment(
    ///         JWTToken::from("1234"),
    ///         &ExtensionId::from("5678"),
    ///         &[Segment::Broadcaster],
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-configuration-segment>
    fn get_extension_configuration_segment<'a>(
        &'a self,
        jwt_token: JWTToken,
        extension_id: &'a ExtensionId,
        segments: &'a [Segment],
    ) -> GetExtensionConfigurationSegmentBuilder<'a>;

    /// Updates a configuration segment
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    /// * `segment` -
    ///
    /// # Returns
    ///
    /// Returns a [`SetExtensionConfigurationSegmentBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::{ExtensionsAPI, Segment},
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .set_extension_configuration_segment(
    ///         &ExtensionId::from("1234"),
    ///         Segment::Broadcaster,
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-configuration-segment>
    fn set_extension_configuration_segment<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
        segment: Segment,
    ) -> SetExtensionConfigurationSegmentBuilder<'a>;

    /// Updates the extension’s required_configuration string
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    /// * `extension_id` -
    /// * `extension_version` -
    /// * `required_configuration` -
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{BroadcasterId, ExtensionId}
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .set_extension_required_configuration(
    ///         &BroadcasterId::from("1234"),
    ///         &ExtensionId::from("5678"),
    ///         "extension_version",
    ///         "required_configuration"
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#set-extension-required-configuration>
    fn set_extension_required_configuration(
        &self,
        broadcaster_id: &BroadcasterId,
        extension_id: &ExtensionId,
        extension_version: &str,
        required_configuration: &str,
    ) -> TwitchAPIRequest<NoContent>;

    /// Sends a message to one or more viewers
    ///
    /// # Arguments
    ///
    /// * `targets` -
    /// * `broadcaster_id` -
    /// * `message` -
    /// * `is_global_broadcast` -
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .send_extension_pubsub_message(
    ///         &["target"],
    ///         "message",
    ///         &BroadcasterId::from("1234"),
    ///         None
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-pubsub-message>
    fn send_extension_pubsub_message(
        &self,
        targets: &[&str],
        message: &str,
        broadcaster_id: &BroadcasterId,
        is_global_broadcast: Option<bool>,
    ) -> TwitchAPIRequest<NoContent>;

    /// Gets a list of broadcasters that are streaming live and have installed or activated the extension
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    /// * `pagination` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetExtensionLiveChannelsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_extension_live_channels(&ExtensionId::from("5678"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-live-channels>
    fn get_extension_live_channels<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionLiveChannelsBuilder<'a>;

    /// Gets an extension’s list of shared secrets
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`ExtensionSecretsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_extension_secrets(&ExtensionId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-secrets>
    fn get_extension_secrets(
        &self,
        extension_id: &ExtensionId,
    ) -> TwitchAPIRequest<ExtensionSecretsResponse>;

    /// Creates a shared secret used to sign and verify JWT tokens
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    /// * `delay` -
    ///
    /// # Returns
    ///
    /// Returns a [`ExtensionSecretsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_extension_secret(&ExtensionId::from("1234"), None)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-extension-secret>
    fn create_extension_secret(
        &self,
        extension_id: &ExtensionId,
        delay: Option<u64>,
    ) -> TwitchAPIRequest<ExtensionSecretsResponse>;

    /// Sends a message to the specified broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    /// * `text` - (max 280)
    /// * `extension_id` -
    /// * `extension_version` -
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{BroadcasterId, ExtensionId}
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .send_extension_chat_message(
    ///         &BroadcasterId::from("1234"),
    ///         "text",
    ///         &ExtensionId::from("5678"),
    ///         "extension_version"
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-extension-chat-message>
    fn send_extension_chat_message(
        &self,
        broadcaster_id: &BroadcasterId,
        text: &str,
        extension_id: &ExtensionId,
        extension_version: &str,
    ) -> TwitchAPIRequest<NoContent>;

    /// Gets information about an extension
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    /// * `extension_version` -
    ///
    /// # Returns
    ///
    /// Returns a [`ExtensionsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_extensions(&ExtensionId::from("1234"), None)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extensions>
    fn get_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<ExtensionsResponse>;

    /// Gets information about a released extension
    ///
    /// # Arguments
    ///
    /// * `extension_id` -
    /// * `extension_version` -
    ///
    /// # Returns
    ///
    /// Returns a [`ExtensionsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_released_extensions(&ExtensionId::from("1234"), None)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-released-extensions>
    fn get_released_extensions(
        &self,
        extension_id: &ExtensionId,
        extension_version: Option<&str>,
    ) -> TwitchAPIRequest<ExtensionsResponse>;

    /// Gets the list of Bits products that belongs to the extension
    ///
    /// # Arguments
    ///
    /// * `should_inclue_all` -
    ///
    /// # Returns
    ///
    /// Returns a [`ExtensionsBitsProductsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::extensions::ExtensionsAPI;
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_extension_bits_products(None)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-bits-products>
    fn get_extension_bits_products(
        &self,
        should_inclue_all: Option<bool>,
    ) -> TwitchAPIRequest<ExtensionsBitsProductsResponse>;

    /// Adds or updates a Bits product that the extension created
    ///
    /// # Arguments
    ///
    /// * `sku` -
    /// * `cost` -
    /// * `display_name` -
    /// * `opts` -
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateExtensionBitsProductBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{Cost, CostType}
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_extension_bits_product(
    ///         "sku",
    ///         Cost::new(5000, CostType::Bits),
    ///         "display_name",
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-extension-bits-product>
    fn update_extension_bits_product<'a>(
        &'a self,
        sku: &'a str,
        cost: Cost,
        display_name: &'a str,
    ) -> UpdateExtensionBitsProductBuilder<'a>;
}

impl ExtensionsAPI for TwitchAPI {
    fn get_extension_configuration_segment<'a>(
        &'a self,
        jwt_token: JWTToken,
        extension_id: &'a ExtensionId,
        segments: &'a [Segment],
    ) -> GetExtensionConfigurationSegmentBuilder<'a> {
        GetExtensionConfigurationSegmentBuilder::new(self, jwt_token, extension_id, segments)
    }
    fn set_extension_configuration_segment<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
        segment: Segment,
    ) -> SetExtensionConfigurationSegmentBuilder<'a> {
        SetExtensionConfigurationSegmentBuilder::new(self, extension_id, segment)
    }
    simple_endpoint!(
            fn set_extension_required_configuration(
                broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
                extension_id: &ExtensionId [skip],
                extension_version: &str [skip],
                required_configuration: &str [skip],
            ) -> NoContent;
            endpoint: SetExtensionRequiredConfiguration,
            method: PUT,
            path: [EXTENSIONS, REQUIRED_CONFIGURATION],
            headers: [json],
            body: {
                serde_json::to_string(&SetExtensionRequiredConfigurationBody {
                    extension_id,
                    extension_version,
                    required_configuration
                }).ok()
        }
    );
    simple_endpoint!(
        fn send_extension_pubsub_message(
            targets: &[&str] [skip],
            message: &str [skip],
            broadcaster_id: &BroadcasterId [skip],
            is_global_broadcast: Option<bool> [skip],
        ) -> NoContent;
            endpoint: SendExtensionPubSubMessage,
            method: POST,
            path: [EXTENSIONS, PUBSUB],
            headers: [json],
            body: {
                let body =  SendExtensionPubSubMessageBody{
                    target:targets,
                    message,
                    broadcaster_id,
                    is_global_broadcast,
                };
                serde_json::to_string(&body).ok()
            }
    );
    fn get_extension_live_channels<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionLiveChannelsBuilder<'a> {
        GetExtensionLiveChannelsBuilder::new(self, extension_id)
    }
    simple_endpoint!(
    fn get_extension_secrets(
        extension_id: &ExtensionId [key = EXTENSION_ID],
    ) -> ExtensionSecretsResponse;
        endpoint: GetExtensionSecrets,
        method: GET,
        path: [EXTENSIONS, JWT, SECRETS],
    );
    simple_endpoint!(
    fn create_extension_secret(
        extension_id: &ExtensionId [key = EXTENSION_ID],
        delay: Option<u64> [opt, key = DELAY, convert = to_string],
    ) -> ExtensionSecretsResponse;
        endpoint: CreateExtensionSecret,
        method: POST,
        path: [EXTENSIONS, JWT, SECRETS],
    );
    simple_endpoint!(
    fn send_extension_chat_message(
        broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
        text: &str [skip],
        extension_id: &ExtensionId [skip],
        extension_version: &str [skip],
    ) -> NoContent;
        endpoint: SendExtensionChatMessage,
        method: POST,
        path: [EXTENSIONS, CHAT],
        headers: [json],
        // body: {ExtensionChatMessageIntoRequestBody::new(text, extension_id, extension_version).into_json()}
        body: {
            Some(
                serde_json::json!({
                    "text":text,
                    EXTENSION_ID:extension_id,
                    EXTENSION_VERSION:extension_version
                }).to_string()
            )
        }
    );
    simple_endpoint!(
    fn get_extensions(
        extension_id: &ExtensionId [key = EXTENSION_ID],
        extension_version: Option<&str> [opt, key = EXTENSION_VERSION],
    ) -> ExtensionsResponse;
        endpoint: GetExtensions,
        method: GET,
        path: [EXTENSIONS],
    );
    simple_endpoint!(
    fn get_released_extensions(
        extension_id: &ExtensionId [key = EXTENSION_ID],
        extension_version: Option<&str> [opt, key = EXTENSION_VERSION],
    ) -> ExtensionsResponse;
        endpoint: GetReleasedExtensions,
        method: GET,
        path: [EXTENSIONS, RELEASED],
    );
    simple_endpoint!(
    fn get_extension_bits_products(
        should_include_all: Option<bool> [opt, key = SHOULD_INCLUDE_ALL, convert = to_string],
    ) -> ExtensionsBitsProductsResponse;
        endpoint: GetExtensionBitsProducts,
        method: GET,
        path: [BITS, EXTENSIONS],

    );
    fn update_extension_bits_product<'a>(
        &'a self,
        sku: &'a str,
        cost: Cost,
        display_name: &'a str,
    ) -> UpdateExtensionBitsProductBuilder<'a> {
        UpdateExtensionBitsProductBuilder::new(self, sku, cost, display_name)
    }
}
