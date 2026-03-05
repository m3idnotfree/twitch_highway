//! Twitch EventSub WebSocket client and router.
//!
//! This module provides a WebSocket client for receiving Twitch EventSub notifications in real-time,
//! along with a router for handling different event types. The design is similar to axum's routing pattern.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//!
//! use tokio::sync::RwLock;
//! use twitch_highway::{
//!     eventsub::{
//!         events::channels::follow::ChannelFollow,
//!         websocket::{
//!             self,
//!             extract::{Event, State},
//!             routes::{channel_follow, welcome},
//!             Router,
//!         },
//!     },
//!     types::SessionId,
//! };
//!
//! #[derive(Clone)]
//! struct AppState {
//!     session_id: Arc<RwLock<SessionId>>,
//! }
//!
//! async fn on_welcome() {
//!     println!("Connected to Twitch EventSub!");
//! }
//!
//! async fn on_follow(Event(event): Event<ChannelFollow>, State(state): State<AppState>) {
//!     println!("New follower: {}", event.user_name);
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let state = AppState {
//!         session_id: Arc::new(RwLock::new(SessionId::from("initial"))),
//!     };
//!
//!     let app = Router::new()
//!         .route(welcome(on_welcome))
//!         .route(channel_follow(on_follow))
//!         .with_state(state);
//!
//!     websocket::client("wss://eventsub.wss.twitch.tv/ws", app)
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! # Router and Handlers
//!
//! The router system works similarly to axum:
//!
//! - Define handlers as async functions
//! - Use extractors to access request data (Event, Session, State, etc.)
//! - Route messages to handlers based on message type or subscription type
//! - Apply middleware layers for logging, rate limiting, etc.
//!
//! ## Extractors
//!
//! ```rust
//! use twitch_highway::eventsub::{
//!     events::hype_train,
//!     websocket::extract::{Event, Meta, Session, State},
//! };
//!
//! # #[derive(Clone)]
//! # struct AppState;
//!
//! async fn handler(
//!     Event(event): Event<hype_train::v2::HypeTrainBegin>,  // The event data
//!     Meta(meta): Meta,                   // Message metadata
//!     Session(session): Session,          // Session information (for welcome/reconnect)
//!     State(state): State<AppState>,      // Application state
//! ) {
//!
//! }
//! ```
//!
//! ## Middleware Layers
//!
//! ```rust
//! use twitch_highway::eventsub::websocket::{layer::TraceLayer, Router, routes::welcome};
//! # async fn example(state:String) {
//! let app = Router::new()
//!     .route(welcome(on_welcome))
//!     .layer(TraceLayer::new())
//!     .with_state(state);
//!
//! # twitch_highway::eventsub::websocket::client("wss://eventsub.wss.twitch.tv/ws", app).await.unwrap();
//! # }
//!
//! # async fn on_welcome() {}
//! ```
//!
//! # Reconnection Handling
//!
//! The client automatically handles:
//! - Keepalive messages (sent every 10 seconds by Twitch)
//! - Reconnection requests from the server
//! - Connection failures with exponential backoff

#[cfg(feature = "websocket")]
mod router;

#[cfg(feature = "websocket")]
mod client;

mod common;
mod messages;
mod scanner;
mod types;

#[cfg(feature = "websocket")]
pub use client::{client, Client, Config, Error, WithGracefulShutdown};

#[cfg(feature = "websocket")]
pub use router::{extract, layer, routes, Router};

pub use common::{IntoResponse, Request, Response};
pub use messages::*;
pub use scanner::{ScanError, Scanner};
pub use types::*;
