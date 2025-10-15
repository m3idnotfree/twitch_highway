use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use axum::{
    self,
    body::Body,
    body::Bytes,
    extract::State,
    http::{header::HeaderMap, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use tokio::{
    net::TcpListener,
    sync::{oneshot, Mutex},
};
use twitch_highway::eventsub::webhook::{
    get_message_type, get_subscription_type, verify_event_message, Challenge, MessageType,
    Notification, Revoke, VerificationError,
};
use twitch_highway::eventsub::SubscriptionType;

pub async fn axum_server(
    secret: String,
    listener: TcpListener,
    shutdown_rx: oneshot::Receiver<()>,
    verify_tx: oneshot::Sender<bool>,
    event_count: Arc<AtomicUsize>,
    received_events: Arc<Mutex<Vec<String>>>,
) {
    let state = AppState {
        secret,
        verify_tx: Arc::new(Mutex::new(Some(verify_tx))),
        event_count,
        received_events,
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
) -> Result<Response, VerificationError> {
    let is_valid = verify_event_message(&headers, &body, &state.secret).is_ok();

    if let Some(tx) = state.verify_tx.lock().await.take() {
        let _ = tx.send(is_valid);
    }

    if !is_valid {
        verify_event_message(&headers, &body, &state.secret)?;
    }

    if let Ok(message_type) = get_message_type(&headers) {
        match message_type {
            MessageType::Verification => {
                let challenge: Challenge = serde_json::from_slice(&body).unwrap();

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(challenge.challenge))
                    .unwrap())
            }
            MessageType::Notification => {
                state.event_count.fetch_add(1, Ordering::SeqCst);

                let sub_type = get_subscription_type(&headers);
                if let Some(sub_type) = sub_type {
                    {
                        let mut events = state.received_events.lock().await;
                        events.push(format!("{:?}", sub_type));
                    }

                    match sub_type {
                        SubscriptionType::AutomodMessageHold => {}
                        SubscriptionType::AutomodMessageHoldV2 => {}
                        _ => {}
                    };
                }

                let _notification: Notification<serde_json::Value> =
                    serde_json::from_slice(&body).unwrap();

                Ok(Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Body::empty())
                    .unwrap())
            }
            MessageType::Revocation => {
                state.event_count.fetch_add(1, Ordering::SeqCst);

                {
                    let mut events = state.received_events.lock().await;
                    events.push("Revocation".to_string());
                }

                let _revocation: Revoke = serde_json::from_slice(&body).unwrap();

                Ok(Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Body::empty())
                    .unwrap())
            }
        }
    } else {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap())
    }
}

#[derive(Clone)]
struct AppState {
    secret: String,
    verify_tx: Arc<Mutex<Option<oneshot::Sender<bool>>>>,
    event_count: Arc<AtomicUsize>,
    received_events: Arc<Mutex<Vec<String>>>,
}
