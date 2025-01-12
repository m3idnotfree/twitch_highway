use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, UserId, AFTER, BROADCASTER_ID, FIRST, USER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait SubscriptionsAPI: TwitchAPIBase {
    fn get_broadcaster_subscriptions(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        first: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn check_user_subscpition(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl SubscriptionsAPI for TwitchAPI {
    fn get_broadcaster_subscriptions(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        first: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["subscriptions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_id.map(|ids| ids.iter().map(|id| (USER_ID, id))))
            .query_opt(FIRST, first)
            .query_opt(AFTER, after)
            .query_opt("before", before);

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
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["subscriptions", "user"])
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
