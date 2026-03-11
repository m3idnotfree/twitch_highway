#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    subscriptions::SubscriptionsAPI,
    types::{BroadcasterId, UserId},
};

#[tokio::test]
async fn get_broadcaster_subscriptions() {
    let suite = HttpMock::new().await;
    suite.get_broadcaster_subscriptions().await;

    let result = suite
        .api()
        .get_broadcaster_subscriptions(&BroadcasterId::from("141981764"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn check_user_subscription() {
    let suite = HttpMock::new().await;
    suite.check_user_subscription().await;

    let result = suite
        .api()
        .check_user_subscription(
            &BroadcasterId::from("149747285"),
            &UserId::from("141981764"),
        )
        .await;

    assert!(result.is_ok());
}

// api_test!(
//     get_broadcaster_subscriptions | api | {
//         api.get_broadcaster_subscriptions(&BroadcasterId::from("141981764"))
//     }
// );
//
// api_test!(
//     check_user_subscription [&BroadcasterId::from("149747285"), &UserId::from("141981764")]
// );
