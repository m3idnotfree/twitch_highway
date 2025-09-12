mod condition;
mod request;
mod response;
mod subscription;
mod subscription_types;
mod transport;

pub use condition::Condition;
pub use request::{CreateEventSubRequest, GetEventRequest};
pub use response::{CreateEventSubscriptionsResponse, EventSubData, EventSubscriptionsResponse};
pub use subscription::Subscription;
pub use subscription_types::SubscriptionType;
pub use transport::{Transport, TransportMethod, TransportType};

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
            req: CreateEventSubRequest<Condition, T>,
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

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{
        eventsub::{Condition, CreateEventSubRequest, EventSubAPI, SubscriptionType, Transport},
        types::{ConduitId, SessionId, SubscriptionId, UserId},
    };

    api_test!(
        create_eventsub,
        [CreateEventSubRequest::new(
            SubscriptionType::UserUpdate,
            Condition::new().user_id(UserId::from("1234")),
            Transport::webhook(
                Url::parse("https://this-is-a-callback.com").unwrap(),
                "s3cre7".to_string()
            )
        )]
    );
    api_test!(
        delete_eventsub,
        [&SubscriptionId::from(
            "26b1c993-bfcf-44d9-b876-379dacafe75a"
        )]
    );
    api_test!(get_eventsub, [None, None]);

    api_test!(extra
        create_eventsub,
        create_eventsub2,
        [CreateEventSubRequest::new(
            SubscriptionType::UserUpdate,
            Condition::new().user_id(UserId::from("1234")),
            Transport::websocket(
                SessionId::from("AQoQexAWVYKSTIu4ec_2VAxyuhAB")
            )
        )]
    );
    api_test!(extra
        create_eventsub,
        create_eventsub3,
        [CreateEventSubRequest::new(
            SubscriptionType::UserUpdate,
            Condition::new().user_id(UserId::from("1234")),
            Transport::conduit(
                ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac")
            )
        )]
    );
}
