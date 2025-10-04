mod response;
mod types;

pub use response::{BroadcasterSubscriptionResponse, UserSubscriptionResponse};
pub use types::{Subscription, Tier};

use crate::types::{
    constants::{BROADCASTER_ID, SUBSCRIPTIONS, USER_ID},
    BroadcasterId, PaginationQuery, UserId,
};

endpoints! {
    SubscriptionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
        fn get_broadcaster_subscriptions(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: Option<&[UserId]>,
            pagination: Option<PaginationQuery>,
        ) -> BroadcasterSubscriptionResponse {
            endpoint_type: GetBroadcasterSubscriptions,
            method: GET,
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
            endpoint_type: CheckUserSubscriptions,
            method: GET,
            path: [SUBSCRIPTIONS, "user"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }
    }
}
