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

pub use builder::{CreateEventSub, GetEventSub, TransportRequest};
pub use condition::Condition;
pub use response::{
    CreateEventSubscriptionsResponse, EventSubscriptionsResponse, TransportResponse,
};
pub use subscription::Subscription;
pub use subscription_types::SubscriptionType;

#[allow(unused_imports)]
pub(crate) use resolve_subscription_type;

use std::future::Future;

use url::Url;

use crate::{
    types::{
        constants::{EVENTSUB, ID, SUBSCRIPTIONS},
        ConduitId, SessionId, SubscriptionId,
    },
    Client, Error,
};

pub trait EventSubAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn webhook_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        callback: Url,
        secret: impl Into<String>,
    ) -> CreateEventSub<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn websocket_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        session_id: SessionId,
    ) -> CreateEventSub<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn conduit_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        conduit_id: ConduitId,
    ) -> CreateEventSub<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription>
    fn delete_eventsub(
        &self,
        subscription_id: &SubscriptionId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    fn get_eventsub<'a>(&'a self) -> GetEventSub<'a>;
}

impl EventSubAPI for Client {
    fn webhook_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        callback: Url,
        secret: impl Into<String>,
    ) -> CreateEventSub<'a> {
        CreateEventSub::webhook(self, kind, callback, secret.into())
    }

    fn websocket_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        session_id: SessionId,
    ) -> CreateEventSub<'a> {
        CreateEventSub::websocket(self, kind, session_id)
    }

    fn conduit_subscription<'a>(
        &'a self,
        kind: SubscriptionType,
        conduit_id: ConduitId,
    ) -> CreateEventSub<'a> {
        CreateEventSub::conduit(self, kind, conduit_id)
    }

    async fn delete_eventsub(&self, subscription_id: &SubscriptionId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, SUBSCRIPTIONS]);

        url.query_pairs_mut().append_pair(ID, subscription_id);

        self.no_content(self.http_client().delete(url)).await
    }

    fn get_eventsub<'a>(&'a self) -> GetEventSub<'a> {
        GetEventSub::new(self)
    }
}
