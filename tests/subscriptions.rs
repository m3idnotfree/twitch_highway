#![cfg(feature = "subscriptions")]

#[macro_use]
mod common;

use twitch_highway::{
    subscriptions::SubscriptionsAPI,
    types::{BroadcasterId, UserId},
};

api_test!(
    get_broadcaster_subscriptions | api | {
        api.get_broadcaster_subscriptions(&BroadcasterId::from("141981764"))
    }
);

api_test!(
    check_user_subscription [&BroadcasterId::from("149747285"), &UserId::from("141981764")]
);
