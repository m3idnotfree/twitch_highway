mod builder;
mod response;
mod types;

pub use builder::GetBroadcasterSubscriptions;
pub use response::{BroadcasterSubscriptionResponse, UserSubscriptionResponse};
pub use types::{Subscription, Tier};

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, SUBSCRIPTIONS, USER, USER_ID},
        BroadcasterId, UserId,
    },
    Client, Error,
};

pub trait SubscriptionsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
    fn get_broadcaster_subscriptions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBroadcasterSubscriptions<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    fn check_user_subscription(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<UserSubscriptionResponse, Error>> + Send;
}

impl SubscriptionsAPI for Client {
    fn get_broadcaster_subscriptions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBroadcasterSubscriptions<'a> {
        GetBroadcasterSubscriptions::new(self, broadcaster_id)
    }

    async fn check_user_subscription(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> Result<UserSubscriptionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SUBSCRIPTIONS, USER]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(USER_ID, user_id);

        self.json(self.http_client().get(url)).await
    }
}
