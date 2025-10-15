use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};
use twitch_highway::{
    eventsub::websocket::{
        self,
        extract::{Session, State as WsState},
        layer::TraceLayer,
        routes::{channel_ban, welcome},
        Router as WsRouter,
    },
    types::SessionId,
};

pub async fn run(session_tx: oneshot::Sender<SessionId>) -> Result<(), websocket::Error> {
    let state = WsAppState {
        session_tx: Arc::new(Mutex::new(Some(session_tx))),
    };
    let app = WsRouter::new()
        .route(welcome(ws_handler))
        .route(channel_ban(channel_ban_handler))
        .layer(TraceLayer::new())
        .with_state(state);

    websocket::client("ws://127.0.0.1:8080/ws", app).await
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
