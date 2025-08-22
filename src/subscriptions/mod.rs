use asknothingx2_util::api::Method;
use response::{BroadcasterSubscriptionResponse, UserSubscriptionResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, SUBSCRIPTIONS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod response;
pub mod types;

endpoints! {
    SubscriptionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
        fn get_broadcaster_subscriptions(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: Option<&[UserId]>,
            pagination: Option<PaginationQuery>,
        ) -> BroadcasterSubscriptionResponse {
            endpoint_type: EndpointType::GetBroadcasterSubscriptions,
            method: Method::GET,
            path: [SUBSCRIPTIONS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                extend(user_id.unwrap_or(&[]).iter().map(|id| (USER_ID, id))),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
        fn check_user_subscription(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: &UserId,
        ) -> UserSubscriptionResponse {
            endpoint_type: EndpointType::CheckUserSubscriptions,
            method: Method::GET,
            path: [SUBSCRIPTIONS, "user"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        subscriptions::SubscriptionsAPI,
        types::{BroadcasterId, UserId},
    };

    api_test!(
        get_broadcaster_subscriptions,
        [&BroadcasterId::new("141981764"), None, None]
    );
    api_test!(
        check_user_subscription,
        [&BroadcasterId::new("149747285"), &UserId::new("141981764")]
    );
}
