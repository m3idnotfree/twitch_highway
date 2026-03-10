use anyhow::Result;
use axum::{
    Router,
    extract::{Json, Query, State},
    response::Redirect,
    routing::get,
};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::broadcast};
use tracing::{error, info, instrument, warn};
use twitch_highway::{Client, chat::ChatAPI};
use twitch_oauth_token::{AccessToken, AuthCallback, AuthorizationCode, TokenInfo, UserToken};

use crate::{AppState, SharedUserInfo, UserInfo, UserOAuthClient};

pub async fn run(
    port: u16,
    state: AppState,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    let app = Router::new()
        .route("/auth/installed/login", get(installed_login))
        .route("/auth/callback", get(auth_callback))
        .route("/api/eventsub/start", get(eventsub_websocket_client_start))
        .route("/api/eventsub/end", get(eventsub_websocket_client_end))
        .route("/api/chat/send", get(send_chat_message))
        .with_state(state);

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).await?;

    info!(
        component = "api",
        action = "start",
        addr = %addr,
        login_url = format!("http://localhost:{}/auth/installed/login", port),
        "API server started"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await?;

    info!(component = "api", action = "stop", "API server stopped");
    Ok(())
}

#[instrument(name = "installed.login", skip(client))]
async fn installed_login(State(client): State<UserOAuthClient>) -> Redirect {
    let mut auth_url = client.authorization_url();
    auth_url.scopes_mut().installed_chatbot();

    info!(
        component = "auth",
        action = "generate_url",
        "OAuth authorization URL generated"
    );

    Redirect::to(auth_url.url().as_str())
}

#[instrument(name = "auth.callback", skip(callback, state))]
async fn auth_callback(
    Query(callback): Query<AuthCallback>,
    State(state): State<AppState>,
) -> String {
    let user_token = match exchange_token(&state.oauth_client, callback.code, callback.state).await
    {
        Ok(token) => token,
        Err(msg) => return msg,
    };

    let validate_token = match validate_token(&state.oauth_client, &user_token.access_token).await {
        Ok(token) => token,
        Err(msg) => return msg,
    };

    let username = validate_token.login.clone();
    let user_id = validate_token.user_id.clone();
    let user_info = UserInfo {
        client_id: validate_token.client_id,
        login: validate_token.login,
        user_id: user_id.clone().into(),
        token: user_token,
    };

    *state.user_info.write().await = Some(user_info);

    info!(
        component = "auth",
        action = "complete",
        username = %username,
        user_id = %user_id,
        eventsub_start_url = format!("http://localhost:{}/api/eventsub/start", state.port),
        "Authentication completed. Use /api/eventsub/start to start the EventSub client."
    );

    "Authentication successful. Use /api/eventsub/start to start the EventSub client.".to_string()
}

async fn exchange_token(
    client: &UserOAuthClient,
    code: AuthorizationCode,
    state: String,
) -> Result<UserToken, String> {
    client.exchange_code(code, state).await.map_err(|e| {
        error!(
            component = "auth",
            action = "exchange_token",
            error = %e,
            "Token exchange failed"
        );
        format!("Authentication failed: {:#}", e)
    })
}

async fn validate_token(
    client: &UserOAuthClient,
    access_token: &AccessToken,
) -> Result<TokenInfo, String> {
    client
        .validate_access_token(access_token)
        .await
        .map_err(|e| {
            error!(
                component = "auth",
                action = "validate_token",
                error = %e,
                "Token validation API call failed"
            );
            format!("Authentication failed: {:#}", e)
        })
}

#[instrument(name = "chat.send", skip(user_info, request), fields(message_length = request.message.len()))]
async fn send_chat_message(
    State(user_info): State<SharedUserInfo>,
    Json(request): Json<SendChatRequest>,
) -> String {
    let user_info = user_info.read().await;
    let Some(user) = user_info.as_ref() else {
        warn!(
            component = "chat",
            action = "send",
            status = "unauthorized",
            "Chat message send attempted without authentication"
        );
        return "Not authenticated".to_string();
    };

    let api = Client::new(user.token.access_token.clone(), user.client_id.clone());
    match api
        .send_chat_message(
            &user.user_id.to_broadcaster_id(),
            &user.user_id,
            &request.message,
        )
        .json()
        .await
    {
        Ok(_) => {
            info!(
                component = "chat",
                action = "send",
                status = "success",
                "Chat message sent"
            );

            "Message sent successfully".to_string()
        }
        Err(e) => {
            error!(
                component = "chat",
                action = "send",
                status = "error",
                error = %e,
                "Chat message send failed"
            );

            format!("Failed to send message: {}", e)
        }
    }
}

#[instrument(name = "eventsub.start", skip(state))]
async fn eventsub_websocket_client_start(State(state): State<AppState>) -> String {
    let user_info = state.user_info.read().await;
    if user_info.is_none() {
        warn!(
            component = "eventsub",
            action = "start",
            status = "unauthorized",
            "EventSub start attempted without authentication"
        );
        return "Not authenticated. Please login first at /auth/installed/login".to_string();
    }

    info!(
        component = "eventsub",
        action = "start",
        status = "triggered",
        "EventSub WebSocket client start triggered"
    );
    state.trigger_eventsub_start();

    "EventSub WebSocket client starting...".to_string()
}

#[instrument(name = "eventsub.end", skip(state))]
async fn eventsub_websocket_client_end(State(state): State<AppState>) -> String {
    info!(
        component = "eventsub",
        action = "end",
        status = "triggered",
        "EventSub WebSocket client end triggered"
    );
    state.trigger_eventsub_end();

    "EventSub WebSocket client stopping...".to_string()
}

#[derive(Debug, Deserialize)]
pub struct SendChatRequest {
    pub message: String,
}
