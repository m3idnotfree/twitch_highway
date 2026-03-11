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

use types::{
    ExtensionChatMessageIntoRequestBody, SendExtensionPubSubMessageBody,
    SetExtensionRequiredConfigurationBody,
};

use std::future::Future;

use crate::{
    types::{
        constants::{
            BITS, BROADCASTER_ID, CHAT, DELAY, EXTENSIONS, EXTENSION_ID, EXTENSION_VERSION, JWT,
            PUBSUB, RELEASED, REQUIRED_CONFIGURATION, SECRETS, SHOULD_INCLUDE_ALL,
        },
        BroadcasterId, Cost, ExtensionId, JWTToken,
    },
    Client, Error,
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::{ExtensionsAPI, Segment},
    ///     types::{ExtensionId, JWTToken},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extension_configuration_segment(
    ///         JWTToken::from("1234"),
    ///         &ExtensionId::from("5678"),
    ///         &[Segment::Broadcaster],
    ///     )
    ///     .send()
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::{ExtensionsAPI, Segment},
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .set_extension_configuration_segment(
    ///         &ExtensionId::from("1234"),
    ///         Segment::Broadcaster,
    ///     )
    ///     .send()
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
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{BroadcasterId, ExtensionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .set_extension_required_configuration(
    ///         &BroadcasterId::from("1234"),
    ///         &ExtensionId::from("5678"),
    ///         "extension_version",
    ///         "required_configuration"
    ///     )
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
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a message to one or more viewers
    ///
    /// # Arguments
    ///
    /// * `targets` -
    /// * `broadcaster_id` -
    /// * `message` -
    /// * `is_global_broadcast` -
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_extension_pubsub_message(
    ///         &["target"],
    ///         "message",
    ///         &BroadcasterId::from("1234"),
    ///         None
    ///     )
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
    ) -> impl Future<Output = Result<(), Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extension_live_channels(&ExtensionId::from("5678"))
    ///     .send()
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extension_secrets(&ExtensionId::from("1234"))
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
    ) -> impl Future<Output = Result<ExtensionSecretsResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_extension_secret(&ExtensionId::from("1234"), None)
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
    ) -> impl Future<Output = Result<ExtensionSecretsResponse, Error>> + Send;

    /// Sends a message to the specified broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    /// * `text` - (max 280)
    /// * `extension_id` -
    /// * `extension_version` -
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{BroadcasterId, ExtensionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_extension_chat_message(
    ///         &BroadcasterId::from("1234"),
    ///         "text",
    ///         &ExtensionId::from("5678"),
    ///         "extension_version"
    ///     )
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
    ) -> impl Future<Output = Result<(), Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extensions(&ExtensionId::from("1234"), None)
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
    ) -> impl Future<Output = Result<ExtensionsResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::ExtensionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_released_extensions(&ExtensionId::from("1234"), None)
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
    ) -> impl Future<Output = Result<ExtensionsResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::extensions::ExtensionsAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extension_bits_products(None)
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
    ) -> impl Future<Output = Result<ExtensionsBitsProductsResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     extensions::ExtensionsAPI,
    ///     types::{Cost, CostType}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_extension_bits_product(
    ///         "sku",
    ///         Cost::new(5000, CostType::Bits),
    ///         "display_name",
    ///     )
    ///     .send()
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

impl ExtensionsAPI for Client {
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
    ) -> GetExtensionLiveChannelsBuilder<'a> {
        GetExtensionLiveChannelsBuilder::new(self, extension_id)
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
    ) -> UpdateExtensionBitsProductBuilder<'a> {
        UpdateExtensionBitsProductBuilder::new(self, sku, cost, display_name)
    }
}
