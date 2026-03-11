#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    eventsub::{EventSubAPI, SubscriptionType},
    types::{ConduitId, SessionId, SubscriptionId, UserId},
};
use url::Url;

#[tokio::test]
async fn create_eventsub() {
    let suite = HttpMock::new().await;
    suite.create_eventsub().await;

    let result = suite
        .api()
        .webhook_subscription(
            SubscriptionType::UserUpdate,
            Url::parse("https://this-is-a-callback.com").unwrap(),
            "s3cre7",
        )
        .user_id(UserId::from("1234"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_eventsub() {
    let suite = HttpMock::new().await;
    suite.delete_eventsub().await;

    let result = suite
        .api()
        .delete_eventsub(&SubscriptionId::from(
            "26b1c993-bfcf-44d9-b876-379dacafe75a",
        ))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_eventsub() {
    let suite = HttpMock::new().await;
    suite.get_eventsub().await;

    let result = suite.api().get_eventsub().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_eventsub2() {
    let suite = HttpMock::new().await;
    suite.create_eventsub2().await;

    let result = suite
        .api()
        .websocket_subscription(
            SubscriptionType::UserUpdate,
            SessionId::from("AQoQexAWVYKSTIu4ec_2VAxyuhAB"),
        )
        .user_id(UserId::from("1234"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_eventsub3() {
    let suite = HttpMock::new().await;
    suite.create_eventsub3().await;

    let result = suite
        .api()
        .conduit_subscription(
            SubscriptionType::UserUpdate,
            ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"),
        )
        .user_id(UserId::from("1234"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[cfg(test)]
#[cfg(any(feature = "webhook-actix", feature = "webhook-axum"))]
mod webhook {
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
    };

    use tokio::{
        net::TcpListener,
        sync::{oneshot, Mutex},
        time::sleep,
    };
    use twitch_highway::eventsub::webhook::generate_secret;

    use crate::common::{axum_server, trigger_webhook_event};

    #[tokio::test]
    async fn webhook_trigger() {
        let secret = generate_secret();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let event_count = Arc::new(AtomicUsize::new(0));
        let event_count_clone = event_count.clone();

        let received_events = Arc::new(Mutex::new(Vec::new()));
        let received_events_clone = received_events.clone();

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (verify_tx, verify_rx) = oneshot::channel();

        tokio::spawn(axum_server(
            secret.clone(),
            listener,
            shutdown_rx,
            verify_tx,
            event_count_clone,
            received_events_clone,
        ));

        let forward_address = format!("http://{}/eventsub", addr);
        let trigger =
            trigger_webhook_event(&forward_address, &secret).with_output_file("trigger_output.log");

        let events = vec![
            "add-moderator",
            "add-redemption",
            "add-reward",
            "ban",
            "channel-gift",
            "charity-donate",
            "charity-start",
            "charity-progress",
            // "charity-end",
            "cheer",
            // "drop",
            "gift",
            "goal-begin",
            "goal-end",
            "goal-progress",
            "grant",
            "hype-train-begin",
            "hype-train-end",
            "hype-train-progress",
            "poll-begin",
            "poll-end",
            "poll-progress",
            "prediction-begin",
            "prediction-end",
            "prediction-lock",
            "prediction-progress",
            "raid",
            "remove-moderator",
            "remove-reward",
            "revoke",
            // "stream-change",
            "streamdown",
            "streamup",
            "subscribe-message",
            "subscribe",
            "transaction",
            "unban",
            "unsubscribe",
            "update-redemption",
            "update-reward",
            "user-update",
        ];

        let expected_count = events.len();

        for event in &events {
            trigger.event(*event).await.unwrap();
        }

        sleep(Duration::from_secs(2)).await;

        let verify_result = verify_rx.await.unwrap();
        assert!(verify_result, "Event signature verification failed");

        let actual_count = event_count.load(Ordering::SeqCst);

        assert_eq!(
            actual_count, expected_count,
            "Expected {} events, but received {}",
            expected_count, actual_count
        );

        let _ = shutdown_tx.send(());
    }

    #[tokio::test]
    async fn webhook_success_verify() {
        let secret = generate_secret();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let event_count = Arc::new(AtomicUsize::new(0));
        let event_count_clone = event_count.clone();

        let received_events = Arc::new(Mutex::new(Vec::new()));
        let received_events_clone = received_events.clone();

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (verify_tx, verify_rx) = oneshot::channel();

        tokio::spawn(axum_server(
            secret.clone(),
            listener,
            shutdown_rx,
            verify_tx,
            event_count_clone,
            received_events_clone,
        ));

        let forward_address = format!("http://{}/eventsub", addr);
        let trigger = trigger_webhook_event(&forward_address, &secret);
        trigger.event("subscribe").await.unwrap();

        let verify_result = verify_rx.await.unwrap();
        assert!(verify_result);

        let _ = shutdown_tx.send(());
    }

    #[tokio::test]
    async fn webhook_failure_verify() {
        let secret = generate_secret();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let event_count = Arc::new(AtomicUsize::new(0));
        let event_count_clone = event_count.clone();

        let received_events = Arc::new(Mutex::new(Vec::new()));
        let received_events_clone = received_events.clone();

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (verify_tx, verify_rx) = oneshot::channel();

        tokio::spawn(axum_server(
            secret.clone(),
            listener,
            shutdown_rx,
            verify_tx,
            event_count_clone,
            received_events_clone,
        ));

        let forward_address = format!("http://{}/eventsub", addr);
        let trigger = trigger_webhook_event(&forward_address, "must_failure");
        trigger.event("subscribe").await.unwrap();

        let verify_result = verify_rx.await.unwrap();
        assert!(!verify_result);

        let _ = shutdown_tx.send(());
    }
}

#[cfg(test)]
#[cfg(feature = "websocket")]
mod websocket {
    use std::time::Duration;

    use tokio::{sync::oneshot, time::sleep};

    use crate::common::{trigger_websocket_event, websocket, CliConfig};

    #[tokio::test]
    async fn websocket() {
        let _cmd = CliConfig::websocket_server()
            .spawn_wait_server()
            .await
            .unwrap();

        let (session_tx, session_rx) = oneshot::channel();
        tokio::spawn(async move { websocket::run(session_tx).await.unwrap() });

        sleep(Duration::from_secs(2)).await;

        let session_id = session_rx.await.unwrap();

        let trigger =
            trigger_websocket_event(&session_id).with_output_file("websocket_trigger.log");

        trigger.event("channel.ban").await.unwrap();
    }
}
