use crate::{
    subscriptions::BroadcasterSubscriptionResponse,
    types::{
        constants::{AFTER, BEFORE, BROADCASTER_ID, FIRST, SUBSCRIPTIONS, USER_ID},
        BroadcasterId, UserId,
    },
};

define_request_builder! {
    #[derive(Debug)]
    GetBroadcasterSubscriptionsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            user_ids: &'a [UserId] [key = USER_ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
            before: &'a str [key = BEFORE]
    }
    } -> BroadcasterSubscriptionResponse;
        endpoint_type: GetBroadcasterSubscriptions,
        method: GET,
        path: [SUBSCRIPTIONS],
}
