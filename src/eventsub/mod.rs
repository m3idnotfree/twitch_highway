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

pub use builder::{CreateEventSub, GetEventSub};
pub use condition::Condition;
pub use response::{
    CreateEventSubscriptionsResponse, EventSubscriptionsResponse, TransportResponse,
};
pub use subscription::Subscription;
pub use subscription_types::SubscriptionType;

#[allow(unused_imports)]
pub(crate) use resolve_subscription_type;

use builder::TransportType;

use std::future::Future;

use crate::{
    types::{
        constants::{EVENTSUB, ID, SUBSCRIPTIONS},
        SubscriptionId,
    },
    Client, Error,
};

pub trait EventSubAPI {
    /// - webhook: `(Url, String)` or `(Url, &str)` - callback URL and secret
    /// - websocket: `SessionId`
    /// - conduit: `ConduitId`
    ///
    /// See <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    fn subscribe<'a>(
        &'a self,
        kind: SubscriptionType,
        transport: impl Into<TransportType>,
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
    fn subscribe<'a>(
        &'a self,
        kind: SubscriptionType,
        transport: impl Into<TransportType>,
    ) -> CreateEventSub<'a> {
        CreateEventSub::new(self, kind, transport)
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
