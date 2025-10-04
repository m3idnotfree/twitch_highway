#![cfg(feature = "eventsub")]

#[macro_use]
mod common;

use std::{sync::Arc, time::Duration};

use axum::{
    self, body::Bytes, extract::State, http::header::HeaderMap, response::IntoResponse,
    routing::post, Router,
};
use common::{trigger_webhook_event, trigger_websocket_event, CliConfig};
use tokio::{
    net::TcpListener,
    sync::{oneshot, Mutex},
};
use twitch_highway::{
    eventsub::{
        webhook::{generate_secret, verify_event_message},
        websocket::{
            self,
            extract::{Session, State as WsState},
            routes::{channel_ban, welcome},
            Router as WsRouter,
        },
        Condition, CreateEventSubRequest, EventSubAPI, SubscriptionType, Transport,
    },
    types::{ConduitId, SessionId, SubscriptionId, UserId},
};
use url::Url;

api_test!(
    create_eventsub,
    [CreateEventSubRequest::new(
        SubscriptionType::UserUpdate,
        Condition::new().user_id(UserId::from("1234")),
        Transport::webhook(
            Url::parse("https://this-is-a-callback.com").unwrap(),
            "s3cre7".to_string()
        )
    )]
);
api_test!(
    delete_eventsub,
    [&SubscriptionId::from(
        "26b1c993-bfcf-44d9-b876-379dacafe75a"
    )]
);
api_test!(get_eventsub, [None, None]);

api_test!(extra
    create_eventsub,
    create_eventsub2,
    [CreateEventSubRequest::new(
        SubscriptionType::UserUpdate,
        Condition::new().user_id(UserId::from("1234")),
        Transport::websocket(
            SessionId::from("AQoQexAWVYKSTIu4ec_2VAxyuhAB")
        )
    )]
);
api_test!(extra
    create_eventsub,
    create_eventsub3,
    [CreateEventSubRequest::new(
        SubscriptionType::UserUpdate,
        Condition::new().user_id(UserId::from("1234")),
        Transport::conduit(
            ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac")
        )
    )]
);

#[tokio::test]
async fn webhook_success_verify() {
    let secret = generate_secret();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (result_tx, result_rx) = oneshot::channel();

    tokio::spawn(axum_server(
        secret.clone(),
        listener,
        shutdown_rx,
        result_tx,
    ));

    let forward_address = format!("http://{}/eventsub", addr);
    let trigger = trigger_webhook_event(&forward_address, &secret);
    trigger.event("subscribe").await.unwrap();

    let verify_result = result_rx.await.unwrap();
    assert!(verify_result);

    let _ = shutdown_tx.send(());
}

#[tokio::test]
async fn webhook_failure_verify() {
    let secret = generate_secret();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (result_tx, result_rx) = oneshot::channel();

    tokio::spawn(axum_server(
        secret.clone(),
        listener,
        shutdown_rx,
        result_tx,
    ));

    let forward_address = format!("http://{}/eventsub", addr);
    let trigger = trigger_webhook_event(&forward_address, "must_failure");
    trigger.event("subscribe").await.unwrap();

    let verify_result = result_rx.await.unwrap();
    assert!(!verify_result);

    let _ = shutdown_tx.send(());
}

async fn axum_server(
    secret: String,
    listener: TcpListener,
    shutdown_rx: oneshot::Receiver<()>,
    result_tx: oneshot::Sender<bool>,
) {
    let state = AppState {
        secret,
        result_tx: Arc::new(Mutex::new(Some(result_tx))),
    };

    let app = Router::new()
        .route("/eventsub", post(handler))
        .with_state(state);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await
        .unwrap()
}

async fn handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let verify = verify_event_message(&headers, &body, &state.secret);
    let is_valid = verify.is_ok();

    if let Some(tx) = state.result_tx.lock().await.take() {
        let _ = tx.send(is_valid);
    }
}

#[derive(Clone)]
struct AppState {
    secret: String,
    result_tx: Arc<Mutex<Option<oneshot::Sender<bool>>>>,
}

#[tokio::test]
async fn websocket() {
    let _cmd = CliConfig::websocket_server()
        .spawn_wait_server()
        .await
        .unwrap();

    let (session_tx, session_rx) = oneshot::channel();
    let state = WsAppState {
        session_tx: Arc::new(Mutex::new(Some(session_tx))),
    };
    let app = WsRouter::new()
        .route(welcome(ws_handler))
        .route(channel_ban(channel_ban_handler))
        .with_state(state);

    tokio::spawn(async move {
        websocket::client("ws://127.0.0.1:8080/ws", app)
            .await
            .unwrap()
    });

    tokio::time::sleep(Duration::from_secs(2)).await;

    let session_id = session_rx.await.unwrap();

    let trigger = trigger_websocket_event(&session_id);

    trigger.event("channel.ban").await.unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;
}

#[derive(Clone)]
struct WsAppState {
    session_tx: Arc<Mutex<Option<oneshot::Sender<SessionId>>>>,
}
async fn ws_handler(Session(s): Session, WsState(state): WsState<WsAppState>) {
    if let Some(tx) = state.session_tx.lock().await.take() {
        let _ = tx.send(s.id);
    }
}

async fn channel_ban_handler() {}
