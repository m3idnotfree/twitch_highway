//! # twitch_highway
//!
//! A Rust library for the Twitch Helix API with type safety and comprehensive EventSub support.
//!
//! Official Twitch API Documentation: <https://dev.twitch.tv/docs/api/reference/>
//!
//! ## Getting Started
//!
//! **Important:** By default, no API endpoints are enabled. You must specify the features you need in `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! twitch_highway = { version = "0.3", features = ["moderation", "chat"] }
//! tokio = { version = "1", features = ["full"] }
//! asknothingx2-util = { version = "0.1", features = ["oauth"] }
//! ```
//! ### Basic Usage
//!
//! ```rust
//! # #[cfg(feature = "moderation")]
//! # {
//! use asknothingx2_util::oauth::{AccessToken, ClientId};
//! use twitch_highway::{
//!     moderation::ModerationAPI,
//!     types::{BroadcasterId, ModeratorId, UserId},
//!     TwitchAPI,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let api = TwitchAPI::new(
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
//!     .duration(600) // Optional
//!     .reason("no reason") // Optional: 10 minues
//!     .json()
//!     .await?;
//!
//! println!("User banned successfully");
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ## Error Handling
//!
//! ```rust
//! # use twitch_highway::request::TwitchAPIRequest;
//! # use serde::de::DeserializeOwned;
//! # async fn example<T: DeserializeOwned>(api: TwitchAPIRequest<T>) {
//! match api.json().await {
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
//! - `webhook-verify`: Core verification only
//! - `webhook-http`: Generic HTTP header support
//! - `webhook-axum`: Axum framework support
//! - `webhook-actix`: Actix-web framework support
//! - `webhook`: All webhook features
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
//! - `websocket-client`: Client with reconnection
//! - `websocket-router`: Event router with handles
//! - `websocket`:  All WebSocket features
//!
//! See the [`eventsub`] module documentation for more details.
//!
//! ## OAuth Token Management
//!
//! This library is designed to work with [`twitch_oauth_token`](https://docs.rs/twitch_oauth_token) for OAuth authentication:
//!
//! ```toml
//! [dependencies]
//! twitch_oauth_token = "2.0"
//! ```
//!
//! ## API Endpoint Features
//!
//! # Features
//! - [`ads`][crate::ads::AdsAPI]
//! - [`analytics`][crate::analytics::AnalyticsAPI]
//! - [`bits`][crate::bits::BitsAPI]
//! - [`channels`][crate::channels::ChannelsAPI]
//! - [`channel-points`][crate::channel_points::ChannelPointsAPI]
//! - [`charity`][crate::charity::CharityAPI]
//! - [`chat`][crate::chat::ChatAPI]
//! - [`clips`][crate::clips::ClipsAPI]
//! - [`ccls`][crate::ccls::CclsAPI]
//! - [`conduits`][crate::conduits::ConduitsAPI]
//! - [`entitlements`][crate::entitlements::EntitlementsAPI]
//! - [`extensions`][crate::extensions::ExtensionsAPI]
//! - [`eventsub`][crate::eventsub::EventSubAPI]
//! - [`games`][crate::games::GamesAPI]
//! - [`goals`][crate::goals::GoalsAPI]
//! - [`guest-star`][crate::guest_star::GuestStarAPI]
//! - [`hype-train`][crate::hype_train::HypeTrainAPI]
//! - [`moderation`][crate::moderation::ModerationAPI]
//! - [`polls`][crate::polls::PollsAPI]
//! - [`predictions`][crate::predictions::PredictionsAPI]
//! - [`raid`][crate::raid::RaidAPI]
//! - [`schedule`][crate::schedule::ScheduleAPI]
//! - [`search`][crate::search::SearchAPI]
//! - [`streams`][crate::streams::StreamsAPI]
//! - [`subscriptions`][crate::subscriptions::SubscriptionsAPI]
//! - [`teams`][crate::teams::TeamsAPI]
//! - [`users`][crate::users::UserAPI]
//! - [`videos`][crate::videos::VideosAPI]
//! - [`whisper`][crate::whisper::WhisperAPI]

#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod request;
pub mod types;

mod error;

pub use error::Error;

use std::sync::LazyLock;

use asknothingx2_util::{
    api::{preset, HeaderMut},
    oauth::{AccessToken, ClientId},
};
use reqwest::{header::HeaderMap, Client};
use url::Url;

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";

static BASE_URL: LazyLock<Url> = LazyLock::new(|| url::Url::parse(TWITCH_API_BASE).unwrap());

#[derive(Debug, Clone)]
pub struct TwitchAPI {
    access_token: AccessToken,
    client_id: ClientId,
    url: Url,
    client: Client,
}

impl TwitchAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
            url: BASE_URL.clone(),
            client: preset::rest_api("twitch-highway/1.0")
                .build_client()
                .unwrap(),
        }
    }

    pub fn with_client(access_token: AccessToken, client_id: ClientId, client: Client) -> Self {
        Self {
            access_token,
            client_id,
            url: BASE_URL.clone(),
            client,
        }
    }

    pub fn with_url(access_token: AccessToken, client_id: ClientId, url: Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
            client: preset::rest_api("twitch-highway/1.0")
                .build_client()
                .unwrap(),
        }
    }

    pub fn set_access_token(mut self, access_token: AccessToken) -> Self {
        self.access_token = access_token;
        self
    }

    pub fn set_client_id(mut self, client_id: ClientId) -> Self {
        self.client_id = client_id;
        self
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    pub fn set_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub(crate) fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap();
        headers
    }

    #[cfg(any(
        feature = "ads",
        feature = "channel-points",
        feature = "channels",
        feature = "chat",
        feature = "conduits",
        feature = "extensions",
        feature = "entitlements",
        feature = "eventsub",
        feature = "moderation",
        feature = "polls",
        feature = "predictions",
        feature = "schedule",
        feature = "streams",
        feature = "users",
        feature = "whisper",
    ))]
    pub(crate) fn header_json(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap()
            .content_type_json();
        headers
    }

    #[cfg(feature = "extensions")]
    pub(crate) fn build_jwt_headers(&self, jwt: &crate::types::JWTToken) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers).bearer_token(jwt.as_str());
        headers
    }

    pub(crate) fn build_url(&self) -> Url {
        self.url.clone()
    }
}

#[cfg(feature = "ads")]
pub mod ads;
#[cfg(feature = "analytics")]
pub mod analytics;
#[cfg(feature = "bits")]
pub mod bits;
#[cfg(feature = "ccls")]
pub mod ccls;
#[cfg(feature = "channel-points")]
pub mod channel_points;
#[cfg(feature = "channels")]
pub mod channels;
#[cfg(feature = "charity")]
pub mod charity;
#[cfg(feature = "chat")]
pub mod chat;
#[cfg(feature = "clips")]
pub mod clips;
#[cfg(feature = "conduits")]
pub mod conduits;
#[cfg(feature = "entitlements")]
pub mod entitlements;
#[cfg(feature = "eventsub")]
pub mod eventsub;
#[cfg(feature = "extensions")]
pub mod extensions;
#[cfg(feature = "games")]
pub mod games;
#[cfg(feature = "goals")]
pub mod goals;
#[cfg(feature = "guest-star")]
pub mod guest_star;
#[cfg(feature = "hype-train")]
pub mod hype_train;
#[cfg(feature = "moderation")]
pub mod moderation;
#[cfg(feature = "polls")]
pub mod polls;
#[cfg(feature = "predictions")]
pub mod predictions;
#[cfg(feature = "raid")]
pub mod raid;
#[cfg(feature = "schedule")]
pub mod schedule;
#[cfg(feature = "search")]
pub mod search;
#[cfg(feature = "streams")]
pub mod streams;
#[cfg(feature = "subscriptions")]
pub mod subscriptions;
#[cfg(feature = "teams")]
pub mod teams;
#[cfg(feature = "users")]
pub mod users;
#[cfg(feature = "videos")]
pub mod videos;
#[cfg(feature = "whisper")]
pub mod whisper;

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "eventsub",
    feature = "moderation"
))]
mod serde_helpers;
