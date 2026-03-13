//! A Rust library for the Twitch Helix API with type safety and comprehensive EventSub support.
//!
//! Official Twitch API Documentation: <https://dev.twitch.tv/docs/api/reference/>
//!
//! ## Getting Started
//!
//! ### Basic Usage
//!
//! ```no_run
//! use twitch_highway::{
//!     moderation::ModerationAPI,
//!     types::{BroadcasterId, ModeratorId, UserId},
//!     AccessToken, ClientId,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let api = twitch_highway::Client::new(
//!     AccessToken::from("your_access_token"),
//!     ClientId::from("your_client_id"),
//! );
//!
//! let response = api
//!     .ban_user(
//!         &BroadcasterId::from("12345"),
//!         &ModeratorId::from("67890"),
//!         &UserId::from("54321"),
//!     )
//!     .duration(600) // Optional: 10 minutes
//!     .reason("no reason") // Optional
//!     .send()
//!     .await?;
//!
//! println!("User banned successfully");
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! ```no_run
//! # use twitch_highway::{types::UserId, videos::VideosAPI, Client};
//! # async fn example(api: Client) {
//! match api.get_videos(&UserId::from("123")).send().await {
//!     Ok(response) => {
//!         // Process successful response
//!     }
//!     Err(e) => {
//!         if e.is_request() {
//!             // Network or connection error
//!             eprintln!("Request failed: {}", e);
//!         } else if e.is_api() {
//!             // Twitch API returned an error (4xx, 5xx)
//!             eprintln!("API error: {}", e);
//!         } else if e.is_decode() {
//!             // Failed to parse JSON response
//!             eprintln!("JSON decode error: {}", e);
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## EventSub Integration
//!
//! Comprehensive support for both Webhook and WebSocket transports.
//!
//! ### Webhook Verification
//!
//! - HMAC-SHA256 signature verification
//! - Framework integrations: axum, actix-web
//! - Custom implementations via [`HeaderAccess`](crate::eventsub::webhook::HeaderAccess) trait
//!
//! #### Feature Flags
//!
//! - `webhook-axum`: Axum framework support
//! - `webhook-actix`: Actix-web framework support
//!
//! ### WebSocket Client
//!
//! - Automatic reconnection with exponential backoff
//! - Keepalive message handling
//! - Type-safe event routing (similar to axum)
//! - Middleware support (logging, rate limiting)
//!
//! #### Feature Flags
//!
//! - `websocket`: WebSocket client with reconnection and event routing
//!
//! See the [`eventsub`] module documentation for more details.
//!
//! ## OAuth Token Management
//!
//! This library is designed to work with [`twitch_oauth_token`](https://docs.rs/twitch_oauth_token) for OAuth authentication.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod types;

mod client;
mod error;
mod serde_helpers;

pub use client::Client;
pub use error::Error;

pub mod ads;
pub mod analytics;
pub mod bits;
pub mod ccls;
pub mod channel_points;
pub mod channels;
pub mod charity;
pub mod chat;
pub mod clips;
pub mod conduits;
pub mod entitlements;
pub mod eventsub;
pub mod extensions;
pub mod games;
pub mod goals;
pub mod guest_star;
pub mod hype_train;
pub mod moderation;
pub mod polls;
pub mod predictions;
pub mod raid;
pub mod schedule;
pub mod search;
pub mod streams;
pub mod subscriptions;
pub mod teams;
pub mod users;
pub mod videos;
pub mod whisper;

pub use asknothingx2_util::oauth::{AccessToken, ClientId};
