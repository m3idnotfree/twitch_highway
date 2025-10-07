#[cfg(feature = "webhook")]
pub mod webhook;
#[cfg(feature = "websocket")]
pub mod websocket;

pub mod events;

#[macro_use]
mod subscription_types;

mod condition;
mod request;
mod response;
mod subscription;
mod transport;

pub use condition::Condition;
pub use request::{CreateEventSubRequest, GetEventRequest};
pub use response::{CreateEventSubscriptionsResponse, EventSubscriptionsResponse};
pub use subscription::Subscription;
pub use subscription_types::SubscriptionType;
pub use transport::{Transport, TransportType};

pub(crate) use resolve_subscription_type;

use crate::{
    request::NoContent,
    types::{
        constants::{EVENTSUB, ID, SUBSCRIPTIONS},
        PaginationQuery, SubscriptionId,
    },
};

endpoints! {
    EventSubAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
        fn create_eventsub<T: TransportType>(
            &self,
            req: CreateEventSubRequest<T>,
        ) -> CreateEventSubscriptionsResponse {
            endpoint_type: CreateEventSub,
            method: POST,
            path: [EVENTSUB, SUBSCRIPTIONS],
            headers: [json],
            body: req.into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription>
        fn delete_eventsub(
            &self,
            subscription_id: &SubscriptionId,
        ) -> NoContent {
            endpoint_type: DeleteEventSub,
            method: DELETE,
            path: [EVENTSUB, SUBSCRIPTIONS],
            query_params: {
                query(ID, subscription_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
        fn get_eventsub(
            &self,
            opts: Option<GetEventRequest>,
            pagination: Option<PaginationQuery>,
        ) -> EventSubscriptionsResponse {
            endpoint_type: GetEventSub,
            method: GET,
            path: [EVENTSUB, SUBSCRIPTIONS],
            query_params: {
                opt_into_query(opts),
                pagination(pagination)
            }
        }
    }
}
