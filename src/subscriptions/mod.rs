mod builder;
mod response;
mod types;

pub use builder::GetBroadcasterSubscriptionsBuilder;
pub use response::{BroadcasterSubscriptionResponse, UserSubscriptionResponse};
pub use types::{Subscription, Tier};

use crate::types::{
    constants::{BROADCASTER_ID, SUBSCRIPTIONS, USER, USER_ID},
    BroadcasterId, UserId,
};

use crate::request::TwitchAPIRequest;
use crate::TwitchAPI;

pub trait SubscriptionsAPI {
    /// Gets a list of users that subscribe to the specified broadcaster
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - ID. This ID must match the user ID in the access token.
    ///
    /// # Returns
    ///
    /// Returns a [`GetBroadcasterSubscriptionsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     subscriptions::SubscriptionsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_broadcaster_subscriptions(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:subscriptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
    fn get_broadcaster_subscriptions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBroadcasterSubscriptionsBuilder<'a>;

    /// Checks whether the user subscribes to the broacaster’s channel
    ///
    /// # Arguments
    ///
    /// * `broacaster_id` - The ID of a partner or affiliate broadcaster.
    /// * `user_id` - The ID of the user that you’re checking to see whether they subscribe to the broadcaster in broadcaster_id.
    ///
    /// # Returns
    ///
    /// Returns a [`UserSubscriptionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     subscriptions::SubscriptionsAPI,
    ///     types::{BroadcasterId, UserId}
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .check_user_subscription(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678")
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
    /// `user:read:subscriptions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    fn check_user_subscription(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> TwitchAPIRequest<UserSubscriptionResponse>;
}

impl SubscriptionsAPI for TwitchAPI {
    fn get_broadcaster_subscriptions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBroadcasterSubscriptionsBuilder<'a> {
        GetBroadcasterSubscriptionsBuilder::new(self, broadcaster_id)
    }
    simple_endpoint!(
        fn check_user_subscription(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
            user_id: &UserId [key = USER_ID],
        ) -> UserSubscriptionResponse;
            endpoint: CheckUserSubscriptions,
            method: GET,
            path: [SUBSCRIPTIONS, USER],
    //             query_params: {
    //                 query(BROADCASTER_ID, broadcaster_id),
    //                 query(USER_ID, user_id)
    //             }

    );
}
