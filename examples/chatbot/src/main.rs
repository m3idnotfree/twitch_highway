//! # Twitch Chatbot Setup
//!
//! ## Required Environment Variables (.env file)
//!
//! Create a `.env` file in the project root with the following variables:
//!
//! - **CLIENT_ID**
//! - **CLIENT_SECRET**
//! - **REDIRECT_URI**: The OAuth redirect URI for this chatbot
//!   - This chatbot runs on localhost, so the REDIRECT_URI must match the localhost URL
//!   - Example: `http://localhost:3000/auth/callback`
//!   - **Important**: You must register this exact URL in the Twitch Developer Console
//!     (Application OAuth Redirect URLs) for the OAuth authentication to work
//! - **PORT**
mod api;
mod eventsub;

use std::{env, str::FromStr, sync::Arc};

use anyhow::{Context, Result};
use axum::extract::FromRef;
use tokio::{
    signal,
    sync::{RwLock, broadcast, watch},
};
use tracing::{error, info};
use twitch_highway::types::{SessionId, UserId};
use twitch_oauth_token::{
    ClientId, ClientSecret, RedirectUrl, TwitchOauth, UserAuth, UserToken, csrf::CsrfConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;

    let (shutdown_tx, _) = broadcast::channel(1);
    let (eventsub_start_tx, _) = watch::channel(false);
    let (eventsub_end_tx, _) = watch::channel(false);

    let state = AppState::new(
        config.init_oauth_client()?,
        eventsub_start_tx,
        eventsub_end_tx,
        config.port,
    );

    let shutdown_task = {
        let shutdown_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            shutdown_signal(shutdown_tx).await;
        })
    };

    let api_task = {
        let state = state.clone();
        let shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            if let Err(e) = api::run(config.port, state, shutdown_rx).await {
                error!(
                    component = "api",
                    error = %e,
                    "API server terminated unexpectedly"
                );
            }
        })
    };

    let event_task = {
        let state = state.clone();
        let shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            if let Err(e) = eventsub::run_loop(state, shutdown_rx).await {
                error!(
                    component = "eventsub",
                    error = %e,
                    "EventSub task terminated unexpectedly"
                );
            }
        })
    };

    let _ = tokio::join!(shutdown_task, api_task, event_task);

    info!(
        component = "main",
        action = "stop",
        "Chatbot service stopped"
    );
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Config {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub redirect_uri: String,
    pub port: u16,
}

impl Config {
    pub fn init_oauth_client(&self) -> Result<TwitchOauth<UserAuth>> {
        let mut oauth =
            TwitchOauth::from_credentials(self.client_id.clone(), self.client_secret.clone())
                .with_redirect_uri(RedirectUrl::from_str(&self.redirect_uri)?);

        oauth.set_csrf_config(CsrfConfig::new(0, 180));

        Ok(oauth)
    }

    pub fn from_env() -> Result<Config> {
        dotenvy::dotenv().ok();

        let client_id: ClientId = env::var("CLIENT_ID")
            .context("CLIENT_ID environment variable not set")?
            .into();

        let client_secret: ClientSecret = env::var("CLIENT_SECRET")
            .context("CLIENT_SECRET environment variable not set")?
            .into();

        let redirect_uri =
            env::var("REDIRECT_URI").context("REDIRECT_URI environment variable not set")?;

        let port: u16 = env::var("PORT")
            .context("PORT environment variable not set")?
            .parse()
            .context("PORT must be a valid number")?;

        info!(
            component = "config",
            action = "load",
            client_id = %client_id,
            port = %port,
            "Configuration loaded successfully"
        );

        Ok(Config {
            client_id,
            client_secret,
            redirect_uri,
            port,
        })
    }
}

pub type UserOAuthClient = TwitchOauth<UserAuth>;
pub type SharedUserInfo = Arc<RwLock<Option<UserInfo>>>;
pub type SharedSessionId = Arc<RwLock<Option<SessionId>>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub oauth_client: UserOAuthClient,
    pub user_info: SharedUserInfo,
    pub session_id: SharedSessionId,
    pub event_start_tx: watch::Sender<bool>,
    pub event_end_tx: watch::Sender<bool>,
    pub port: u16,
}

impl AppState {
    pub fn new(
        oauth_client: UserOAuthClient,
        event_start_tx: watch::Sender<bool>,
        event_end_tx: watch::Sender<bool>,
        port: u16,
    ) -> Self {
        Self {
            oauth_client,
            user_info: Arc::new(RwLock::new(None)),
            session_id: Arc::new(RwLock::new(None)),
            event_start_tx,
            event_end_tx,
            port,
        }
    }

    pub async fn set_session_id(&self, session_id: SessionId) {
        *self.session_id.write().await = Some(session_id);
    }

    pub fn trigger_eventsub_start(&self) {
        let _ = self.event_start_tx.send(true);
    }

    pub fn trigger_eventsub_end(&self) {
        let _ = self.event_end_tx.send(true);
    }
}

impl FromRef<AppState> for UserOAuthClient {
    fn from_ref(input: &AppState) -> Self {
        input.oauth_client.clone()
    }
}

impl FromRef<AppState> for SharedUserInfo {
    fn from_ref(input: &AppState) -> Self {
        input.user_info.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub client_id: ClientId,
    pub login: String,
    pub user_id: UserId,
    pub token: UserToken,
}

async fn shutdown_signal(shutdown_tx: broadcast::Sender<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!(
                component = "shutdown",
                action = "initiate",
                reason = "user_interrupt",
                "Shutdown initiated by user (Ctrl+C)"
            );
        },
        _ = terminate => {
            info!(
                component = "shutdown",
                action = "initiate",
                reason = "system_signal",
                "Shutdown initiated by system (SIGTERM)"
            );
        },
    }

    let _ = shutdown_tx.send(());
}
