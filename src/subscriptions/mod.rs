use asknothingx2_util::api::Method;
use response::{BroadcasterSubscriptionResponse, UserSubscriptionResponse};

use crate::{
    base::TwitchAPIBase,
    types::{
        constants::{BROADCASTER_ID, SUBSCRIPTIONS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "subscriptions")))]
pub trait SubscriptionsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-broadcaster-subscriptions>
    fn get_broadcaster_subscriptions(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BroadcasterSubscriptionResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#check-user-subscription>
    fn check_user_subscpition(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, UserSubscriptionResponse>;
}

impl SubscriptionsAPI for TwitchAPI {
    fn get_broadcaster_subscriptions(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BroadcasterSubscriptionResponse> {
        let mut url = self.build_url();
        url.path([SUBSCRIPTIONS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_id.map(|ids| ids.iter().map(|id| (USER_ID, id))))
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetBroadcasterSubscriptions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn check_user_subscpition(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, UserSubscriptionResponse> {
        let mut url = self.build_url();
        url.path([SUBSCRIPTIONS, "user"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::CheckUserSubscriptions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
