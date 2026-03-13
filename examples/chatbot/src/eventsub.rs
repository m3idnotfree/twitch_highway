use anyhow::Result;
use tokio::sync::{broadcast, watch};
use tracing::{error, info, instrument, warn};
use twitch_highway::{
    Client,
    eventsub::{
        EventSubAPI, SubscriptionType,
        events::chat::ChannelChatMessage,
        websocket::{
            self, Router,
            extract::State,
            extract::{Event, Session},
            routes,
        },
    },
};

use crate::AppState;

const TWITCH_EVENTSUB_URL: &str = "wss://eventsub.wss.twitch.tv/ws";

#[instrument(name = "eventsub.loop", skip_all)]
pub async fn run_loop(state: AppState, mut shutdown_rx: broadcast::Receiver<()>) -> Result<()> {
    info!(
        component = "eventsub_loop",
        action = "start",
        "EventSub loop started, waiting for start trigger"
    );

    let mut eventsub_start = state.event_start_tx.subscribe();

    loop {
        tokio::select! {
            _ = eventsub_start.changed() => {
                if *eventsub_start.borrow() {
                    info!(
                        component = "eventsub_loop",
                        action = "start_triggered",
                        "EventSub start triggered"
                    );


                    let state_clone = state.clone();
                    let eventsub_end_clone = state.event_end_tx.subscribe();
                    let shutdown_clone = shutdown_rx.resubscribe();

                    if let Err(e) = run(state_clone, shutdown_clone, eventsub_end_clone).await {
                        error!(
                            component = "eventsub_loop",
                            error = %e,
                            "EventSub client terminated with error"
                        );
                    }

                    info!(
                        component = "eventsub_loop",
                        action = "client_stopped",
                        "EventSub client stopped, waiting for restart trigger"
                    );

                    let _ = state.event_start_tx.send(false);
                }
            }
            _ = shutdown_rx.recv() => {
                info!(
                    component = "eventsub_loop",
                    action = "shutdown",
                    "EventSub loop shutting down"
                );
                break;
            }
        }
    }

    Ok(())
}

#[instrument(
    name = "eventsub.run",
    skip(state, shutdown_rx, eventsub_end),
    fields(url = TWITCH_EVENTSUB_URL)
)]
pub async fn run(
    state: AppState,
    mut shutdown_rx: broadcast::Receiver<()>,
    mut eventsub_end: watch::Receiver<bool>,
) -> Result<()> {
    info!(
        component = "eventsub",
        action = "start",
        "Starting EventSub WebSocket client"
    );

    let app = Router::new()
        .route(routes::welcome(on_welcome))
        .route(routes::channel_chat_message(on_chat_message))
        .with_state(state);

    match websocket::client(TWITCH_EVENTSUB_URL, app)
        .with_graceful_shutdown(async move {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    info!(
                        component = "eventsub",
                        action = "shutdown",
                        "EventSub client shutting down (shutdown signal)"
                    );
                }
                _ = eventsub_end.changed() => {
                    if *eventsub_end.borrow() {
                        info!(
                            component = "eventsub",
                            action = "end_triggered",
                            "EventSub client shutting down (end trigger)"
                        );
                    }
                }
            }
        })
        .await
    {
        Ok(_) => {
            info!(
                component = "eventsub",
                action = "stopped",
                "EventSub client stopped gracefully"
            );
            Ok(())
        }
        Err(e) => {
            error!(
                component = "eventsub",
                action = "error",
                error = %e,
                "EventSub client error"
            );
            Err(e.into())
        }
    }
}

#[instrument(
    name = "eventsub.welcome",
    skip(state, session),
    fields(
        session_id = %session.id,
        keepalive_timeout = session.keepalive_timeout_seconds
    )
)]
async fn on_welcome(Session(session): Session, State(state): State<AppState>) {
    info!(
        component = "eventsub",
        action = "welcome",
        session_id = %session.id,
        "Received Welcome message"
    );

    state.set_session_id(session.id.clone()).await;

    let Some(user_info) = &*state.user_info.read().await else {
        warn!(
            component = "eventsub",
            action = "welcome",
            status = "no_user",
            "No authenticated user, skipping subscription"
        );
        return;
    };

    let api = Client::new(
        user_info.token.access_token.clone(),
        user_info.client_id.clone(),
    );

    match api
        .subscribe(SubscriptionType::ChannelChatMessage, session.id.clone())
        .broadcaster_user_id(user_info.user_id.to_broadcaster_id())
        .user_id(user_info.user_id.clone())
        .send()
        .await
    {
        Ok(resp) => {
            if let Some(subscription) = resp.data.first() {
                info!(
                    component = "eventsub",
                    action = "subscription",
                    status = "created",
                    subscription_id = ?subscription.id,
                    subscription_type = "channel.chat.message",
                    subscription_status = ?subscription.status,
                    user_id = %user_info.user_id,
                    username = %user_info.login,
                    "EventSub subscription created"
                );
            }
        }
        Err(e) => {
            error!(
                component = "eventsub",
                action = "subscription",
                status = "error",
                error = %e,
                subscription_type = "channel.chat.message",
                user_id = %user_info.user_id,
                "EventSub subscription creation failed"
            );
        }
    }
}

#[instrument(
    name = "eventsub.chat_message",
    skip(event, _state),
    fields(
        username = %event.chatter_user_name,
        user_id = %event.chatter_user_id,
        message_length = event.message.text.len()
    )
)]
async fn on_chat_message(Event(event): Event<ChannelChatMessage>, State(_state): State<AppState>) {
    info!(
        component = "eventsub",
        action = "chat_message",
        username = %event.chatter_user_name,
        message = %event.message.text,
        "Chat message received"
    );
}
