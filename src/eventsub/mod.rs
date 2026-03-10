#[cfg(any(feature = "webhook-actix", feature = "webhook-axum"))]
pub mod webhook;
#[cfg(feature = "websocket")]
pub mod websocket;

pub mod events;

#[macro_use]
mod subscription_types;

mod builder;
mod condition;
mod response;
mod subscription;

pub use builder::{CreateEventSubBuilder, GetEventSubBuilder, TransportRequest};
pub use condition::Condition;
pub use response::{
    CreateEventSubscriptionsResponse, EventSubscriptionsResponse, TransportResponse,
};
pub use subscription::Subscription;
pub use subscription_types::SubscriptionType;

#[allow(unused_imports)]
pub(crate) use resolve_subscription_type;
use url::Url;

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{EVENTSUB, ID, SUBSCRIPTIONS},
        ConduitId, SessionId, SubscriptionId,
    },
    Client,
};

pub trait EventSubAPI {
    /// Creates a webhook EventSub subscription
    ///
    /// # Arguments
    ///
    /// * `kind` -
    /// * `callback` -
    /// * `secret` -
    ///
    /// # Returns
    ///
    /// Returns a [`CreateEventSubBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     eventsub::{EventSubAPI, SubscriptionType},
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    /// use url::Url;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .webhook_subscription(
    ///         SubscriptionType::ChannelFollow,
    ///         Url::parse("https://example.com").unwrap(),
    ///         "secret"
    ///     )
    ///     .broadcaster_user_id(BroadcasterId::from("1234"))
    ///     .moderator_user_id(ModeratorId::from("5678"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn webhook_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        callback: Url,
        secret: impl Into<String>,
    ) -> CreateEventSubBuilder<'a>;

    /// Creates a WebSocket EventSub subscription
    ///
    /// # Arguments
    ///
    /// * `kind` -
    /// * `session_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`CreateEventSubBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     eventsub::{EventSubAPI, SubscriptionType},
    ///     types::{BroadcasterId, ModeratorId, SessionId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .websocket_subscription(
    ///         SubscriptionType::ChannelFollow,
    ///         SessionId::from("1234")
    ///     )
    ///     .broadcaster_user_id(BroadcasterId::from("1234"))
    ///     .moderator_user_id(ModeratorId::from("5678"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn websocket_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        session_id: SessionId,
    ) -> CreateEventSubBuilder<'a>;

    /// Creates a conduit EventSub subscription
    ///
    /// # Arguments
    ///
    /// * `kind` -
    /// * `conduit_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`CreateEventSubBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     eventsub::{EventSubAPI, SubscriptionType},
    ///     types::{BroadcasterId, ConduitId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .conduit_subscription(
    ///         SubscriptionType::ChannelFollow,
    ///         ConduitId::from("1234")
    ///     )
    ///     .broadcaster_user_id(BroadcasterId::from("1234"))
    ///     .moderator_user_id(ModeratorId::from("5678"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn conduit_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        conduit_id: ConduitId,
    ) -> CreateEventSubBuilder<'a>;

    /// Deletes an EventSub subscription
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The ID of the subscription to delete.
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     eventsub::EventSubAPI,
    ///     types::SubscriptionId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let subscription_id = SubscriptionId::from("1234");
    /// let response = api
    ///     .delete_eventsub(&subscription_id)
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
    /// <https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription>
    fn delete_eventsub(&self, subscription_id: &SubscriptionId) -> TwitchAPIRequest<NoContent>;

    /// Gets a list of EventSub subscriptions that the client in the access token created
    ///
    /// # Returns
    ///
    /// Returns a [`GetEventSubBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::eventsub::EventSubAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_eventsub()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    fn get_eventsub<'a>(&'a self) -> GetEventSubBuilder<'a>;
}

impl EventSubAPI for Client {
    fn webhook_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        callback: Url,
        secret: impl Into<String>,
    ) -> CreateEventSubBuilder<'a> {
        CreateEventSubBuilder::webhook(self, kind, callback, secret.into())
    }
    fn websocket_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        session_id: SessionId,
    ) -> CreateEventSubBuilder<'a> {
        CreateEventSubBuilder::websocket(self, kind, session_id)
    }
    fn conduit_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        conduit_id: ConduitId,
    ) -> CreateEventSubBuilder<'a> {
        CreateEventSubBuilder::conduit(self, kind, conduit_id)
    }
    fn delete_eventsub(&self, subscription_id: &SubscriptionId) -> TwitchAPIRequest<NoContent> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, SUBSCRIPTIONS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(ID, subscription_id);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::DeleteEventSub,
            url,
            reqwest::Method::DELETE,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
    }
    fn get_eventsub<'a>(&'a self) -> GetEventSubBuilder<'a> {
        GetEventSubBuilder::new(self)
    }
}
